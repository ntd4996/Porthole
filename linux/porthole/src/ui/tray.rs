//! System-tray icon via StatusNotifierItem (`ksni`). Linux counterpart of the
//! macOS `NSStatusItem`. Runs on its own thread and posts `Msg`s back to the
//! GTK main loop.
//!
//! Note: GNOME needs the AppIndicator/KStatusNotifierItem extension for the icon
//! to appear (KDE, most other desktops support it natively).

use crate::app::Msg;
use crate::i18n;

struct PortholeTray {
    tx: async_channel::Sender<Msg>,
}

impl PortholeTray {
    fn send(&self, msg: Msg) {
        let _ = self.tx.send_blocking(msg);
    }
}

impl ksni::Tray for PortholeTray {
    fn id(&self) -> String {
        crate::app::APP_ID.to_string()
    }

    fn title(&self) -> String {
        "Porthole".to_string()
    }

    fn icon_name(&self) -> String {
        crate::app::APP_ID.to_string()
    }

    /// Left-click activates: toggle the popover window.
    fn activate(&mut self, _x: i32, _y: i32) {
        self.send(Msg::ToggleWindow);
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::{MenuItem, StandardItem};
        vec![
            StandardItem {
                label: i18n::tr("Open Porthole").to_string(),
                activate: Box::new(|t: &mut Self| t.send(Msg::ToggleWindow)),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: i18n::tr("Refresh").to_string(),
                activate: Box::new(|t: &mut Self| t.send(Msg::Refresh)),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: i18n::tr("Check for Updates…").to_string(),
                activate: Box::new(|t: &mut Self| t.send(Msg::CheckUpdates)),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: i18n::tr("Quit Porthole").to_string(),
                activate: Box::new(|t: &mut Self| t.send(Msg::Quit)),
                ..Default::default()
            }
            .into(),
        ]
    }
}

/// Spawn the tray service on its own thread.
pub fn start(tx: async_channel::Sender<Msg>) {
    let service = ksni::TrayService::new(PortholeTray { tx });
    service.spawn();
}
