import AppKit

enum Actions {
    static func openInBrowser(port: Int) {
        if let url = URL(string: "http://localhost:\(port)") {
            NSWorkspace.shared.open(url)
        }
    }

    static func openURL(_ string: String) {
        if let url = URL(string: string) { NSWorkspace.shared.open(url) }
    }

    static func copy(_ text: String) {
        NSPasteboard.general.clearContents()
        NSPasteboard.general.setString(text, forType: .string)
    }

    /// SIGTERM the process. Returns false if the call fails.
    @discardableResult
    static func kill(pid: Int32) -> Bool {
        Foundation.kill(pid, SIGTERM) == 0
    }
}
