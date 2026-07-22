//! Popover-style main window. Mirrors `App/ContentView.swift`.

use crate::app::{Controller, Msg, Views};
use crate::ui::{ignored_tab, port_row::PortRowMode, project_group};
use crate::i18n;
use adw::prelude::*;
use porthole_core::{PortInfo, ProjectKind};
use std::rc::Rc;

const WIDTH: i32 = 360;

const CSS: &str = "
.port-num { font-family: monospace; font-weight: bold; color: @accent_color; }
.group-card { background-color: alpha(@window_fg_color, 0.05); border: 1px solid alpha(@window_fg_color, 0.08); border-radius: 10px; }
.kind-badge { background-color: alpha(@window_fg_color, 0.15); border-radius: 8px; padding: 0 6px; }
.tunnel-pill { border-radius: 9px; padding: 1px 7px; }
.tunnel-cloudflare { background-color: alpha(#e8912a, 0.18); color: #c9730a; }
.tunnel-ngrok { background-color: alpha(#2ea043, 0.18); color: #1a7f37; }
.tunnel-tailscale { background-color: alpha(#3b82f6, 0.18); color: #2563eb; }
.tunnel-localtunnel { background-color: alpha(#8b5cf6, 0.18); color: #7c3aed; }
";

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(CSS);
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

pub fn build_window(app: &adw::Application, tx: async_channel::Sender<Msg>) -> Views {
    load_css();
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title(i18n::tr("Porthole"))
        .default_width(WIDTH)
        .resizable(false)
        .hide_on_close(true)
        .build();

    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // --- Header: title + refresh ---
    let header = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    header.set_margin_top(10);
    header.set_margin_bottom(8);
    header.set_margin_start(12);
    header.set_margin_end(12);
    let title = gtk::Label::new(None);
    title.set_markup("<b>Porthole</b>");
    title.set_hexpand(true);
    title.set_halign(gtk::Align::Start);
    let refresh = gtk::Button::from_icon_name("view-refresh-symbolic");
    refresh.add_css_class("flat");
    refresh.set_tooltip_text(Some(i18n::tr("Refresh ports")));
    {
        let tx = tx.clone();
        refresh.connect_clicked(move |_| {
            let _ = tx.send_blocking(Msg::Refresh);
        });
    }
    header.append(&title);
    header.append(&refresh);
    root.append(&header);

    // --- Tabs ---
    let stack = gtk::Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::None);
    let ports_box = gtk::Box::new(gtk::Orientation::Vertical, 8);
    let ignored_box = gtk::Box::new(gtk::Orientation::Vertical, 8);
    for b in [&ports_box, &ignored_box] {
        b.set_margin_top(12);
        b.set_margin_bottom(12);
        b.set_margin_start(12);
        b.set_margin_end(12);
    }
    stack.add_titled(&scrolled(&ports_box), Some("ports"), i18n::tr("Ports"));
    stack.add_titled(&scrolled(&ignored_box), Some("ignored"), i18n::tr("Ignored"));

    let switcher = gtk::StackSwitcher::new();
    switcher.set_stack(Some(&stack));
    switcher.set_margin_start(12);
    switcher.set_margin_end(12);
    switcher.set_margin_bottom(8);
    switcher.set_hexpand(true);
    root.append(&switcher);
    root.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    root.append(&stack);
    root.append(&gtk::Separator::new(gtk::Orientation::Horizontal));

    // --- Footer ---
    let footer = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    footer.set_margin_top(8);
    footer.set_margin_bottom(8);
    footer.set_margin_start(12);
    footer.set_margin_end(12);
    let footer_label = gtk::Label::new(Some(""));
    footer_label.add_css_class("dim-label");
    footer_label.add_css_class("caption");
    footer_label.set_hexpand(true);
    footer_label.set_halign(gtk::Align::Start);
    let updates = gtk::Button::with_label(i18n::tr("Check for Updates"));
    updates.add_css_class("flat");
    updates.set_tooltip_text(Some(i18n::tr("Check for a newer version")));
    {
        let tx = tx.clone();
        updates.connect_clicked(move |_| {
            let _ = tx.send_blocking(Msg::CheckUpdates);
        });
    }
    let quit = gtk::Button::with_label(i18n::tr("Quit"));
    quit.add_css_class("flat");
    {
        let tx = tx.clone();
        quit.connect_clicked(move |_| {
            let _ = tx.send_blocking(Msg::Quit);
        });
    }
    footer.append(&footer_label);
    footer.append(&updates);
    footer.append(&quit);
    root.append(&footer);

    window.set_content(Some(&root));

    Views { window, ports_box, ignored_box, footer_label }
}

fn scrolled(child: &gtk::Box) -> gtk::ScrolledWindow {
    gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .max_content_height(440)
        .propagate_natural_height(true)
        .child(child)
        .build()
}

/// Rebuild both tab pages and the footer from the controller's current state.
pub fn rebuild(controller: &Rc<Controller>) {
    let state = controller.state.borrow();
    let ignore = controller.ignore.borrow();

    let visible: Vec<PortInfo> = state
        .ports
        .iter()
        .filter(|p| !ignore.rules.is_ignored(p))
        .cloned()
        .collect();
    let ignored: Vec<PortInfo> = state
        .ports
        .iter()
        .filter(|p| ignore.rules.is_ignored(p))
        .cloned()
        .collect();
    let tunnel_count: usize = visible.iter().map(|p| p.tunnels.len()).sum();
    drop(state);
    drop(ignore);

    // Ports tab.
    clear(&controller.views.ports_box);
    if !visible.is_empty() {
        for (title, kind, ports) in group(&visible) {
            let card = project_group::build(controller, &title, kind, ports, PortRowMode::Normal);
            controller.views.ports_box.append(&card);
        }
    } else if !controller.state.borrow().did_scan {
        controller.views.ports_box.append(&loading());
    } else {
        controller
            .views
            .ports_box
            .append(&empty_state(i18n::tr("No dev ports running")));
    }

    // Ignored tab.
    clear(&controller.views.ignored_box);
    let ignored_content = ignored_tab::build(controller, &ignored);
    controller.views.ignored_box.append(&ignored_content);

    // Footer.
    controller
        .views
        .footer_label
        .set_text(&i18n::footer(visible.len(), tunnel_count));
}

/// Group ports by project (unprojected under "Other"). Mirrors `PortListView.group`.
pub fn group(ports: &[PortInfo]) -> Vec<(String, Option<ProjectKind>, Vec<PortInfo>)> {
    use std::collections::BTreeMap;
    let mut named: BTreeMap<String, (ProjectKind, Vec<PortInfo>)> = BTreeMap::new();
    let mut other: Vec<PortInfo> = Vec::new();
    for port in ports {
        match &port.project {
            Some(project) => {
                let entry = named
                    .entry(project.name.clone())
                    .or_insert((project.kind, Vec::new()));
                entry.1.push(port.clone());
            }
            None => other.push(port.clone()),
        }
    }
    let mut groups: Vec<(String, Option<ProjectKind>, Vec<PortInfo>)> = named
        .into_iter()
        .map(|(name, (kind, ports))| (name, Some(kind), ports))
        .collect();
    if !other.is_empty() {
        groups.push(("Other".to_string(), None, other));
    }
    groups
}

fn clear(container: &gtk::Box) {
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }
}

fn loading() -> gtk::Widget {
    let b = gtk::Box::new(gtk::Orientation::Vertical, 10);
    b.set_margin_top(28);
    b.set_margin_bottom(28);
    b.set_halign(gtk::Align::Center);
    let spinner = gtk::Spinner::new();
    spinner.start();
    let label = gtk::Label::new(Some(i18n::tr("Scanning ports…")));
    label.add_css_class("dim-label");
    label.add_css_class("caption");
    b.append(&spinner);
    b.append(&label);
    b.upcast()
}

fn empty_state(text: &str) -> gtk::Widget {
    let label = gtk::Label::new(Some(text));
    label.add_css_class("dim-label");
    label.set_margin_top(24);
    label.set_margin_bottom(24);
    label.set_halign(gtk::Align::Center);
    label.set_hexpand(true);
    label.upcast()
}
