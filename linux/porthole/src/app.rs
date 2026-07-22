//! Application controller: owns shared state, drives scans, and wires the tray
//! to the popover window. Linux counterpart of `StatusBarController` +
//! `AppState` + `ScanCoordinator` on macOS.

use crate::ignore_store::IgnoreStore;
use crate::ui::{tray, window};
use crate::{i18n, scan_service, updater};
use adw::prelude::*;
use gtk::glib;
use porthole_core::PortInfo;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub const APP_ID: &str = "org.datnt.Porthole";
const REFRESH_SECS: u64 = 4;

/// Messages posted from background threads / the tray thread onto the GTK main loop.
pub enum Msg {
    Scanned(Vec<PortInfo>),
    ToggleWindow,
    Refresh,
    CheckUpdates,
    Quit,
    UpdateResult(updater::Outcome, bool), // (outcome, silent)
}

#[derive(Default)]
pub struct UiState {
    pub ports: Vec<PortInfo>,
    pub did_scan: bool,
    pub is_scanning: bool,
}

/// Widgets the controller rebuilds on every state change.
pub struct Views {
    pub window: adw::ApplicationWindow,
    pub ports_box: gtk::Box,
    pub ignored_box: gtk::Box,
    pub footer_label: gtk::Label,
}

pub struct Controller {
    pub app: adw::Application,
    pub views: Views,
    pub state: RefCell<UiState>,
    pub ignore: RefCell<IgnoreStore>,
    tx: async_channel::Sender<Msg>,
    refresh_timer: RefCell<Option<glib::SourceId>>,
    hold: RefCell<Option<gtk::gio::ApplicationHoldGuard>>,
}

impl Controller {
    /// Kick off a background scan; the result arrives via `Msg::Scanned`.
    pub fn refresh(self: &Rc<Self>) {
        self.state.borrow_mut().is_scanning = true;
        let tx = self.tx.clone();
        std::thread::spawn(move || {
            let ports = scan_service::scan();
            let _ = tx.send_blocking(Msg::Scanned(ports));
        });
    }

    fn apply_scan(self: &Rc<Self>, ports: Vec<PortInfo>) {
        {
            let mut st = self.state.borrow_mut();
            st.ports = ports;
            st.did_scan = true;
            st.is_scanning = false;
        }
        self.rebuild();
    }

    /// Repopulate both tab pages and the footer from current state + ignore rules.
    pub fn rebuild(self: &Rc<Self>) {
        window::rebuild(self);
    }

    pub fn ignore_process(self: &Rc<Self>, name: &str) {
        self.ignore.borrow_mut().ignore_process(name);
        self.rebuild();
    }

    pub fn ignore_port(self: &Rc<Self>, port: u16) {
        self.ignore.borrow_mut().ignore_port(port);
        self.rebuild();
    }

    pub fn unignore_matching(self: &Rc<Self>, port: &PortInfo) {
        self.ignore.borrow_mut().unignore_matching(port);
        self.rebuild();
    }

    pub fn unignore_process(self: &Rc<Self>, name: &str) {
        self.ignore.borrow_mut().unignore_process(name);
        self.rebuild();
    }

    pub fn unignore_port(self: &Rc<Self>, port: u16) {
        self.ignore.borrow_mut().unignore_port(port);
        self.rebuild();
    }

    /// Confirm, then SIGTERM the process and rescan.
    pub fn confirm_kill(self: &Rc<Self>, pid: i32) {
        let dialog = adw::MessageDialog::new(
            Some(&self.views.window),
            Some(&i18n::kill_pid(pid)),
            None,
        );
        dialog.add_response("cancel", i18n::tr("Cancel"));
        dialog.add_response("kill", i18n::tr("Kill"));
        dialog.set_response_appearance("kill", adw::ResponseAppearance::Destructive);
        dialog.set_default_response(Some("cancel"));
        let this = self.clone();
        dialog.connect_response(None, move |_, resp| {
            if resp == "kill" {
                crate::actions::kill(pid);
                this.refresh();
            }
        });
        dialog.present();
    }

    pub fn toggle_window(self: &Rc<Self>) {
        let win = &self.views.window;
        if win.is_visible() {
            win.set_visible(false);
        } else {
            win.present();
        }
    }

    fn start_timer(self: &Rc<Self>) {
        if self.refresh_timer.borrow().is_some() {
            return;
        }
        let this = self.clone();
        let id = glib::timeout_add_local(Duration::from_secs(REFRESH_SECS), move || {
            this.refresh();
            glib::ControlFlow::Continue
        });
        *self.refresh_timer.borrow_mut() = Some(id);
    }

    fn stop_timer(self: &Rc<Self>) {
        if let Some(id) = self.refresh_timer.borrow_mut().take() {
            id.remove();
        }
    }

    /// `silent` = background check on launch: stay quiet unless an update is found.
    pub fn check_updates(self: &Rc<Self>, silent: bool) {
        let tx = self.tx.clone();
        std::thread::spawn(move || {
            let outcome = updater::check();
            let _ = tx.send_blocking(Msg::UpdateResult(outcome, silent));
        });
    }

    fn handle_update_result(self: &Rc<Self>, outcome: updater::Outcome, silent: bool) {
        use updater::Outcome::*;
        let body = match outcome {
            UpToDate => {
                if silent {
                    return;
                }
                i18n::tr("Up to date").to_string()
            }
            Available { version, applied: true } => {
                format!("Porthole {version} downloaded. Restart to apply.")
            }
            Available { version, applied: false } => {
                format!("{} (v{version})", i18n::tr("Update available"))
            }
            Error(_) => {
                // Manual check: fall back to opening the releases page. Silent: ignore.
                if !silent {
                    crate::actions::open_url(updater::RELEASES_PAGE);
                }
                return;
            }
        };
        let dialog = adw::MessageDialog::new(
            Some(&self.views.window),
            Some(i18n::tr("Check for Updates")),
            Some(&body),
        );
        dialog.add_response("ok", "OK");
        dialog.present();
    }
}

pub fn run() {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        // Guard against building twice if activated again.
        if app.active_window().is_some() {
            return;
        }
        build(app);
    });

    app.run();
}

fn build(app: &adw::Application) {
    let (tx, rx) = async_channel::unbounded::<Msg>();

    let views = window::build_window(app, tx.clone());
    // Hide instead of destroy so the tray can bring it back.
    views.window.connect_close_request(|win| {
        win.set_visible(false);
        glib::Propagation::Stop
    });

    let controller = Rc::new(Controller {
        app: app.clone(),
        views,
        state: RefCell::new(UiState::default()),
        ignore: RefCell::new(IgnoreStore::load()),
        tx: tx.clone(),
        refresh_timer: RefCell::new(None),
        hold: RefCell::new(Some(app.hold())), // keep running with no visible window
    });

    // Auto-refresh only while the popover is on screen.
    {
        let c = controller.clone();
        controller.views.window.connect_map(move |_| {
            c.refresh();
            c.start_timer();
        });
    }
    {
        let c = controller.clone();
        controller.views.window.connect_unmap(move |_| c.stop_timer());
    }

    // System-tray (StatusNotifierItem) on its own thread; talks back via `tx`.
    tray::start(tx.clone());

    // Dispatch messages on the GTK main loop.
    {
        let c = controller.clone();
        glib::spawn_future_local(async move {
            while let Ok(msg) = rx.recv().await {
                match msg {
                    Msg::Scanned(ports) => c.apply_scan(ports),
                    Msg::ToggleWindow => c.toggle_window(),
                    Msg::Refresh => c.refresh(),
                    Msg::CheckUpdates => c.check_updates(false),
                    Msg::UpdateResult(o, silent) => c.handle_update_result(o, silent),
                    Msg::Quit => {
                        c.hold.borrow_mut().take(); // release the keep-alive
                        c.app.quit();
                    }
                }
            }
        });
    }

    // Close the popover when it loses focus (click outside / switch app), like a
    // macOS transient popover. Only after it has actually been focused once, so a
    // window that never grabs focus (some Wayland setups) does not vanish instantly.
    {
        let seen_active = std::cell::Cell::new(false);
        controller.views.window.connect_is_active_notify(move |win| {
            if win.is_active() {
                seen_active.set(true);
            } else if seen_active.replace(false) {
                win.set_visible(false);
            }
        });
    }

    // First paint + first scan, then a quiet background update check.
    controller.rebuild();
    controller.refresh();
    controller.check_updates(true);
}
