//! One port row: number, name, action buttons, tunnel pills, context menu.
//! Mirrors `App/PortRowView.swift`.

use crate::app::Controller;
use crate::{actions, i18n};
use adw::prelude::*;
use porthole_core::{PortInfo, TunnelInfo, TunnelProvider};
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq)]
pub enum PortRowMode {
    Normal,
    Ignored,
}

pub fn build(controller: &Rc<Controller>, port: &PortInfo, mode: PortRowMode) -> gtk::Widget {
    let row = gtk::Box::new(gtk::Orientation::Vertical, 3);
    row.set_margin_start(10);
    row.set_margin_end(10);
    row.set_margin_top(5);
    row.set_margin_bottom(5);

    let top = gtk::Box::new(gtk::Orientation::Horizontal, 8);

    let num = gtk::Label::new(Some(&format!(":{}", port.port)));
    num.add_css_class("port-num");
    num.set_width_chars(6);
    num.set_xalign(0.0);
    top.append(&num);

    let name = gtk::Label::new(Some(&port.display_name));
    name.add_css_class("dim-label");
    name.set_ellipsize(gtk::pango::EllipsizeMode::End);
    name.set_xalign(0.0);
    top.append(&name);

    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    top.append(&spacer);

    // --- Action buttons ---
    let open = icon_button("web-browser-symbolic", i18n::tr("Open in browser"));
    {
        let p = port.port;
        open.connect_clicked(move |_| actions::open_in_browser(p));
    }
    top.append(&open);

    let copy = icon_button("edit-copy-symbolic", i18n::tr("Copy URL"));
    {
        let p = port.port;
        copy.connect_clicked(move |_| actions::copy(&format!("http://localhost:{p}")));
    }
    top.append(&copy);

    match mode {
        PortRowMode::Normal => {
            let hide = icon_button("view-conceal-symbolic", &i18n::ignore_x(&port.command));
            {
                let c = controller.clone();
                let cmd = port.command.clone();
                hide.connect_clicked(move |_| c.ignore_process(&cmd));
            }
            top.append(&hide);

            let kill = icon_button("window-close-symbolic", i18n::tr("Kill process"));
            kill.add_css_class("error");
            {
                let c = controller.clone();
                let pid = port.pid;
                kill.connect_clicked(move |_| c.confirm_kill(pid));
            }
            top.append(&kill);
        }
        PortRowMode::Ignored => {
            let show = icon_button("view-reveal-symbolic", i18n::tr("Un-ignore"));
            {
                let c = controller.clone();
                let p = port.clone();
                show.connect_clicked(move |_| c.unignore_matching(&p));
            }
            top.append(&show);
        }
    }

    row.append(&top);

    // --- Tunnel pills ---
    for tunnel in &port.tunnels {
        row.append(&pill(tunnel));
    }

    // --- Right-click context menu ---
    attach_context_menu(controller, &row, port, mode);

    row.upcast()
}

fn icon_button(icon: &str, tooltip: &str) -> gtk::Button {
    let b = gtk::Button::from_icon_name(icon);
    b.add_css_class("flat");
    b.set_tooltip_text(Some(tooltip));
    b
}

fn provider_class(p: TunnelProvider) -> &'static str {
    match p {
        TunnelProvider::Cloudflare => "tunnel-cloudflare",
        TunnelProvider::Ngrok => "tunnel-ngrok",
        TunnelProvider::Tailscale => "tunnel-tailscale",
        TunnelProvider::Localtunnel => "tunnel-localtunnel",
    }
}

fn pill(tunnel: &TunnelInfo) -> gtk::Widget {
    let content = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    content.add_css_class("tunnel-pill");
    content.add_css_class(provider_class(tunnel.provider));

    let globe = gtk::Image::from_icon_name("network-workgroup-symbolic");
    globe.set_pixel_size(12);
    content.append(&globe);

    let name = gtk::Label::new(Some(tunnel.provider.label()));
    name.add_css_class("caption-heading");
    content.append(&name);

    let detail_text = tunnel
        .public_url
        .clone()
        .unwrap_or_else(|| format!(":{}", tunnel.target_port));
    let detail = gtk::Label::new(Some(&detail_text));
    detail.add_css_class("caption");
    detail.set_ellipsize(gtk::pango::EllipsizeMode::End);
    content.append(&detail);

    let wrapper: gtk::Widget = if let Some(url) = tunnel.public_url.clone() {
        let button = gtk::Button::new();
        button.add_css_class("flat");
        button.set_child(Some(&content));
        button.connect_clicked(move |_| actions::open_url(&url));
        button.upcast()
    } else {
        content.upcast()
    };
    wrapper.set_halign(gtk::Align::Start);
    wrapper.set_margin_start(60);
    wrapper
}

fn attach_context_menu(
    controller: &Rc<Controller>,
    row: &gtk::Box,
    port: &PortInfo,
    mode: PortRowMode,
) {
    let popover = gtk::Popover::new();
    popover.set_has_arrow(false);
    popover.set_parent(row);

    let menu = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let add = |menu: &gtk::Box, label: &str| -> gtk::Button {
        let b = gtk::Button::with_label(label);
        b.add_css_class("flat");
        if let Some(child) = b.child().and_downcast::<gtk::Label>() {
            child.set_xalign(0.0);
        }
        menu.append(&b);
        b
    };

    match mode {
        PortRowMode::Normal => {
            let ip = add(&menu, &i18n::ignore_process(&port.command));
            {
                let c = controller.clone();
                let cmd = port.command.clone();
                let pop = popover.clone();
                ip.connect_clicked(move |_| {
                    pop.popdown();
                    c.ignore_process(&cmd);
                });
            }
            let iport = add(&menu, &i18n::ignore_port(port.port));
            {
                let c = controller.clone();
                let p = port.port;
                let pop = popover.clone();
                iport.connect_clicked(move |_| {
                    pop.popdown();
                    c.ignore_port(p);
                });
            }
        }
        PortRowMode::Ignored => {
            let un = add(&menu, i18n::tr("Un-ignore"));
            {
                let c = controller.clone();
                let p = port.clone();
                let pop = popover.clone();
                un.connect_clicked(move |_| {
                    pop.popdown();
                    c.unignore_matching(&p);
                });
            }
        }
    }
    popover.set_child(Some(&menu));

    let gesture = gtk::GestureClick::new();
    gesture.set_button(gtk::gdk::BUTTON_SECONDARY);
    {
        let pop = popover.clone();
        gesture.connect_pressed(move |_, _, x, y| {
            pop.set_pointing_to(Some(&gtk::gdk::Rectangle::new(x as i32, y as i32, 1, 1)));
            pop.popup();
        });
    }
    row.add_controller(gesture);
}
