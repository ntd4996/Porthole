import SwiftUI
import AppKit

@main
struct PortholeApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate

    var body: some Scene {
        // The UI lives in a status-item popover managed by AppDelegate; this
        // empty scene just satisfies the App protocol.
        Settings { EmptyView() }
    }
}

/// Runs the app as a menu bar accessory (no Dock icon) and starts the status item.
@MainActor
final class AppDelegate: NSObject, NSApplicationDelegate {
    private let statusBar = StatusBarController()

    func applicationDidFinishLaunching(_ notification: Notification) {
        NSApp.setActivationPolicy(.accessory)
        statusBar.start()
        _ = UpdaterController.shared   // start Sparkle background update checks
    }
}
