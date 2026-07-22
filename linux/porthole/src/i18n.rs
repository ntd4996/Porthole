//! Runtime localization. Mirrors the strings in
//! `App/Resources/{en,vi,zh-Hans}.lproj/Localizable.strings`.
//! Language is picked once from `LC_MESSAGES` / `LC_ALL` / `LANG`.

use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq)]
enum Lang {
    En,
    Vi,
    Zh,
}

fn lang() -> Lang {
    static L: OnceLock<Lang> = OnceLock::new();
    *L.get_or_init(|| {
        let raw = std::env::var("LC_ALL")
            .or_else(|_| std::env::var("LC_MESSAGES"))
            .or_else(|_| std::env::var("LANG"))
            .unwrap_or_default()
            .to_lowercase();
        if raw.starts_with("vi") {
            Lang::Vi
        } else if raw.starts_with("zh") {
            Lang::Zh
        } else {
            Lang::En
        }
    })
}

/// Look up a key for the active language, falling back to English.
pub fn tr(key: &str) -> &'static str {
    let table = match lang() {
        Lang::En => EN,
        Lang::Vi => VI,
        Lang::Zh => ZH,
    };
    table
        .iter()
        .chain(EN.iter())
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
        .unwrap_or(key.leak_static())
}

// --- Parameterized helpers ---

pub fn footer(ports: usize, tunnels: usize) -> String {
    tr("footer")
        .replacen("{0}", &ports.to_string(), 1)
        .replacen("{1}", &tunnels.to_string(), 1)
}

pub fn ignore_x(name: &str) -> String {
    tr("ignore_x").replacen("{0}", name, 1)
}

pub fn ignore_process(name: &str) -> String {
    tr("ignore_process").replacen("{0}", name, 1)
}

pub fn ignore_port(port: u16) -> String {
    tr("ignore_port").replacen("{0}", &port.to_string(), 1)
}

pub fn kill_pid(pid: i32) -> String {
    tr("kill_pid").replacen("{0}", &pid.to_string(), 1)
}

/// Small extension so `tr` can return a `'static str` for unknown keys
/// (falls back to the key itself, which is always an English source string).
trait LeakStatic {
    fn leak_static(&self) -> &'static str;
}
impl LeakStatic for str {
    fn leak_static(&self) -> &'static str {
        Box::leak(self.to_string().into_boxed_str())
    }
}

const EN: &[(&str, &str)] = &[
    ("Refresh ports", "Refresh ports"),
    ("Ports", "Ports"),
    ("Ignored", "Ignored"),
    ("Scanning ports…", "Scanning ports…"),
    ("No dev ports running", "No dev ports running"),
    ("Other", "Other"),
    ("footer", "{0} ports · {1} tunnels"),
    ("Check for Updates", "Check for Updates"),
    ("Check for a newer version", "Check for a newer version"),
    ("Quit", "Quit"),
    ("Open in browser", "Open in browser"),
    ("Copy URL", "Copy URL"),
    ("Kill process", "Kill process"),
    ("Un-ignore", "Un-ignore"),
    ("ignore_x", "Ignore {0}"),
    ("ignore_process", "Ignore process {0}"),
    ("ignore_port", "Ignore port {0}"),
    ("kill_pid", "Kill PID {0}?"),
    ("Kill", "Kill"),
    ("Cancel", "Cancel"),
    ("Open Porthole", "Open Porthole"),
    ("Refresh", "Refresh"),
    ("Check for Updates…", "Check for Updates…"),
    ("Quit Porthole", "Quit Porthole"),
    ("Nothing ignored", "Nothing ignored"),
    ("Rules (not running)", "Rules (not running)"),
    ("Ignore a port…", "Ignore a port…"),
    ("Add", "Add"),
    ("Porthole", "Porthole"),
    ("Up to date", "You are on the latest version."),
    ("Update available", "A new version is available."),
];

const VI: &[(&str, &str)] = &[
    ("Refresh ports", "Làm mới cổng"),
    ("Ports", "Cổng"),
    ("Ignored", "Đã ẩn"),
    ("Scanning ports…", "Đang quét cổng…"),
    ("No dev ports running", "Không có cổng dev nào đang chạy"),
    ("Other", "Khác"),
    ("footer", "{0} cổng · {1} tunnel"),
    ("Check for Updates", "Kiểm tra cập nhật"),
    ("Check for a newer version", "Kiểm tra phiên bản mới"),
    ("Quit", "Thoát"),
    ("Open in browser", "Mở trong trình duyệt"),
    ("Copy URL", "Sao chép URL"),
    ("Kill process", "Tắt tiến trình"),
    ("Un-ignore", "Bỏ ẩn"),
    ("ignore_x", "Ẩn {0}"),
    ("ignore_process", "Ẩn tiến trình {0}"),
    ("ignore_port", "Ẩn cổng {0}"),
    ("kill_pid", "Tắt PID {0}?"),
    ("Kill", "Tắt"),
    ("Cancel", "Huỷ"),
    ("Open Porthole", "Mở Porthole"),
    ("Refresh", "Làm mới"),
    ("Check for Updates…", "Kiểm tra cập nhật…"),
    ("Quit Porthole", "Thoát Porthole"),
    ("Nothing ignored", "Chưa ẩn gì"),
    ("Rules (not running)", "Quy tắc (không chạy)"),
    ("Ignore a port…", "Ẩn một cổng…"),
    ("Add", "Thêm"),
    ("Porthole", "Porthole"),
    ("Up to date", "Bạn đang dùng phiên bản mới nhất."),
    ("Update available", "Đã có phiên bản mới."),
];

const ZH: &[(&str, &str)] = &[
    ("Refresh ports", "刷新端口"),
    ("Ports", "端口"),
    ("Ignored", "已忽略"),
    ("Scanning ports…", "正在扫描端口…"),
    ("No dev ports running", "没有正在运行的开发端口"),
    ("Other", "其他"),
    ("footer", "{0} 个端口 · {1} 个隧道"),
    ("Check for Updates", "检查更新"),
    ("Check for a newer version", "检查新版本"),
    ("Quit", "退出"),
    ("Open in browser", "在浏览器中打开"),
    ("Copy URL", "复制链接"),
    ("Kill process", "结束进程"),
    ("Un-ignore", "取消忽略"),
    ("ignore_x", "忽略 {0}"),
    ("ignore_process", "忽略进程 {0}"),
    ("ignore_port", "忽略端口 {0}"),
    ("kill_pid", "结束 PID {0}？"),
    ("Kill", "结束"),
    ("Cancel", "取消"),
    ("Open Porthole", "打开 Porthole"),
    ("Refresh", "刷新"),
    ("Check for Updates…", "检查更新…"),
    ("Quit Porthole", "退出 Porthole"),
    ("Nothing ignored", "没有忽略项"),
    ("Rules (not running)", "规则（未运行）"),
    ("Ignore a port…", "忽略一个端口…"),
    ("Add", "添加"),
    ("Porthole", "Porthole"),
    ("Up to date", "已是最新版本。"),
    ("Update available", "有新版本可用。"),
];
