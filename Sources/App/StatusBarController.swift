import AppKit
import SwiftUI

/// Owns the menu bar status item and a native `NSPopover` (the pattern used by
/// polished menu bar apps): smooth open/close animation, a real arrow pointing
/// at the icon, and transient auto-dismiss on outside clicks.
@MainActor
final class StatusBarController: NSObject, NSPopoverDelegate {
    private var statusItem: NSStatusItem?
    private let popover = NSPopover()
    private let state = AppState()
    private lazy var coordinator = ScanCoordinator(state: state)

    func start() {
        let item = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        let icon = NSImage(systemSymbolName: "circle.circle", accessibilityDescription: "Porthole")
        icon?.isTemplate = true
        item.button?.image = icon
        item.button?.target = self
        item.button?.action = #selector(toggle)
        statusItem = item

        popover.behavior = .transient
        popover.animates = true
        popover.delegate = self
        let host = NSHostingController(rootView: ContentView(state: state) { [weak self] in
            self?.coordinator.refresh()
        })
        host.sizingOptions = [.preferredContentSize]
        popover.contentViewController = host
    }

    @objc private func toggle() {
        guard let button = statusItem?.button else { return }
        if popover.isShown {
            popover.performClose(nil)
        } else {
            popover.show(relativeTo: button.bounds, of: button, preferredEdge: .minY)
        }
    }

    func popoverDidShow(_ notification: Notification) {
        // Drop keyboard focus so no control draws a focus ring on open.
        popover.contentViewController?.view.window?.makeFirstResponder(nil)
        coordinator.menuOpened()
    }
    func popoverDidClose(_ notification: Notification) { coordinator.menuClosed() }
}
