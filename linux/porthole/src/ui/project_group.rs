//! One project card grouping its ports. Mirrors `App/ProjectGroupView.swift`.

use crate::app::Controller;
use crate::i18n;
use crate::ui::port_row::{self, PortRowMode};
use adw::prelude::*;
use gtk::glib;
use porthole_core::{PortInfo, ProjectKind};
use std::rc::Rc;

pub fn build(
    controller: &Rc<Controller>,
    title: &str,
    kind: Option<ProjectKind>,
    ports: Vec<PortInfo>,
    mode: PortRowMode,
) -> gtk::Widget {
    let card = gtk::Box::new(gtk::Orientation::Vertical, 0);
    card.add_css_class("group-card");

    // Header: project name + kind badge.
    let header = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    header.set_margin_start(10);
    header.set_margin_end(10);
    header.set_margin_top(8);
    header.set_margin_bottom(2);

    // "Other" is the catch-all sentinel; project names stay verbatim.
    let shown = if title == "Other" { i18n::tr("Other") } else { title };
    let name = gtk::Label::new(None);
    name.set_markup(&format!("<b>{}</b>", glib::markup_escape_text(shown)));
    name.set_xalign(0.0);
    header.append(&name);

    if let Some(kind) = kind {
        if kind != ProjectKind::Unknown {
            let badge = gtk::Label::new(Some(kind.label()));
            badge.add_css_class("kind-badge");
            badge.add_css_class("caption");
            header.append(&badge);
        }
    }
    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    header.append(&spacer);
    card.append(&header);

    // Port rows with separators between them.
    for (idx, port) in ports.iter().enumerate() {
        if idx > 0 {
            let sep = gtk::Separator::new(gtk::Orientation::Horizontal);
            sep.set_margin_start(10);
            card.append(&sep);
        }
        card.append(&port_row::build(controller, port, mode));
    }

    card.set_margin_bottom(0);
    card.upcast()
}
