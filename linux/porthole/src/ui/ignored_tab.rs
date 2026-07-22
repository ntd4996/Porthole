//! Ignored tab: ignored port cards, a "Rules (not running)" card, and an
//! add-a-port field. Mirrors `App/IgnoredTabView.swift`.

use crate::app::Controller;
use crate::i18n;
use crate::ui::port_row::PortRowMode;
use crate::ui::{project_group, window};
use adw::prelude::*;
use gtk::glib;
use porthole_core::PortInfo;
use std::rc::Rc;

pub fn build(controller: &Rc<Controller>, ignored: &[PortInfo]) -> gtk::Widget {
    let root = gtk::Box::new(gtk::Orientation::Vertical, 8);

    // Ignored ports that are currently running.
    if !ignored.is_empty() {
        for (title, kind, ports) in window::group(ignored) {
            let card = project_group::build(controller, &title, kind, ports, PortRowMode::Ignored);
            root.append(&card);
        }
    }

    // Rules with no currently-running port.
    let (uncovered_procs, uncovered_ports) = uncovered(controller, ignored);
    if !uncovered_procs.is_empty() || !uncovered_ports.is_empty() {
        root.append(&rules_card(controller, &uncovered_procs, &uncovered_ports));
    }

    if ignored.is_empty() && uncovered_procs.is_empty() && uncovered_ports.is_empty() {
        let empty = gtk::Label::new(Some(i18n::tr("Nothing ignored")));
        empty.add_css_class("dim-label");
        empty.set_margin_top(24);
        empty.set_margin_bottom(24);
        empty.set_halign(gtk::Align::Center);
        empty.set_hexpand(true);
        root.append(&empty);
    }

    root.append(&add_port_field(controller));
    root.upcast()
}

/// Rules (process names / ports) not matched by any running ignored port.
fn uncovered(controller: &Rc<Controller>, ignored: &[PortInfo]) -> (Vec<String>, Vec<u16>) {
    let ignore = controller.ignore.borrow();

    let mut procs: Vec<String> = ignore
        .rules
        .processes
        .iter()
        .filter(|name| {
            !ignored.iter().any(|p| {
                name.eq_ignore_ascii_case(&p.command) || name.eq_ignore_ascii_case(&p.display_name)
            })
        })
        .cloned()
        .collect();
    procs.sort();

    let mut ports: Vec<u16> = ignore
        .rules
        .ports
        .iter()
        .filter(|port| !ignored.iter().any(|p| p.port == **port))
        .copied()
        .collect();
    ports.sort_unstable();

    (procs, ports)
}

fn rules_card(controller: &Rc<Controller>, procs: &[String], ports: &[u16]) -> gtk::Widget {
    let card = gtk::Box::new(gtk::Orientation::Vertical, 0);
    card.add_css_class("group-card");

    let title = gtk::Label::new(None);
    title.set_markup(&format!(
        "<b>{}</b>",
        glib::markup_escape_text(i18n::tr("Rules (not running)"))
    ));
    title.set_xalign(0.0);
    title.set_margin_start(10);
    title.set_margin_end(10);
    title.set_margin_top(8);
    title.set_margin_bottom(2);
    card.append(&title);

    for name in procs {
        let c = controller.clone();
        let n = name.clone();
        card.append(&rule_row(name, false, move || c.unignore_process(&n)));
    }
    for port in ports {
        let c = controller.clone();
        let p = *port;
        card.append(&rule_row(&format!(":{port}"), true, move || c.unignore_port(p)));
    }
    card.upcast()
}

fn rule_row<F: Fn() + 'static>(label: &str, mono: bool, remove: F) -> gtk::Widget {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    row.set_margin_start(10);
    row.set_margin_end(10);
    row.set_margin_top(5);
    row.set_margin_bottom(5);

    let text = gtk::Label::new(Some(label));
    if mono {
        text.add_css_class("port-num");
    }
    text.set_ellipsize(gtk::pango::EllipsizeMode::End);
    text.set_xalign(0.0);
    text.set_hexpand(true);
    row.append(&text);

    let btn = gtk::Button::from_icon_name("view-reveal-symbolic");
    btn.add_css_class("flat");
    btn.set_tooltip_text(Some(i18n::tr("Un-ignore")));
    btn.connect_clicked(move |_| remove());
    row.append(&btn);

    row.upcast()
}

fn add_port_field(controller: &Rc<Controller>) -> gtk::Widget {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let entry = gtk::Entry::new();
    entry.set_placeholder_text(Some(i18n::tr("Ignore a port…")));
    entry.set_hexpand(true);
    entry.set_input_purpose(gtk::InputPurpose::Digits);

    let add = gtk::Button::with_label(i18n::tr("Add"));

    let submit = {
        let c = controller.clone();
        let entry = entry.clone();
        move || {
            if let Ok(port) = entry.text().trim().parse::<u16>() {
                entry.set_text("");
                c.ignore_port(port);
            }
        }
    };
    {
        let submit = submit.clone();
        entry.connect_activate(move |_| submit());
    }
    add.connect_clicked(move |_| submit());

    row.append(&entry);
    row.append(&add);
    row.upcast()
}
