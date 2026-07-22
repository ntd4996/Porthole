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

    /// Embedded icon so the tray shows an image even when the themed icon is not
    /// installed (e.g. running the raw AppImage/binary).
    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        icon_pixmap()
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

/// Decode the embedded PNG into the ARGB32 pixmap ksni expects (bytes A,R,G,B).
fn icon_pixmap() -> Vec<ksni::Icon> {
    fn decode() -> Option<ksni::Icon> {
        const PNG: &[u8] = include_bytes!("../../assets/tray-icon.png");
        let decoder = png::Decoder::new(PNG);
        let mut reader = decoder.read_info().ok()?;
        let mut buf = vec![0u8; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).ok()?;
        let channels = match info.color_type {
            png::ColorType::Rgba => 4,
            png::ColorType::Rgb => 3,
            _ => return None,
        };
        let (w, h) = (info.width as usize, info.height as usize);
        let mut data = Vec::with_capacity(w * h * 4);
        for px in buf[..info.buffer_size()].chunks_exact(channels) {
            let (r, g, b) = (px[0], px[1], px[2]);
            let a = if channels == 4 { px[3] } else { 255 };
            data.extend_from_slice(&[a, r, g, b]);
        }
        Some(ksni::Icon { width: w as i32, height: h as i32, data })
    }
    decode().into_iter().collect()
}

/// Spawn the tray service on its own thread.
pub fn start(tx: async_channel::Sender<Msg>) {
    let service = ksni::TrayService::new(PortholeTray { tx });
    service.spawn();
}
