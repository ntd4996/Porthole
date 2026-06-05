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
    private let ignore = IgnoreStore()
    private lazy var coordinator = ScanCoordinator(state: state)

    /// Closes the popover when the user clicks anywhere outside it (including
    /// other apps / the desktop), which a transient popover can miss for a
    /// non-activating menu bar app.
    private var outsideClickMonitor: Any?

    func start() {
        let item = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        let icon = NSImage(systemSymbolName: "circle.circle", accessibilityDescription: "Porthole")
        icon?.isTemplate = true
        item.button?.image = icon
        item.button?.target = self
        item.button?.action = #selector(handleClick)
        item.button?.sendAction(on: [.leftMouseUp, .rightMouseUp])
        statusItem = item

        popover.behavior = .transient
        popover.animates = true
        popover.delegate = self
        let host = NSHostingController(rootView: ContentView(state: state, ignore: ignore) { [weak self] in
            self?.coordinator.refresh()
        })
        host.sizingOptions = [.preferredContentSize]
        popover.contentViewController = host
    }

    /// Left-click toggles the popover; right-click (or control-click) shows a menu.
    @objc private func handleClick() {
        let event = NSApp.currentEvent
        let isRight = event?.type == .rightMouseUp
            || (event?.modifierFlags.contains(.control) ?? false)
        if isRight {
            showMenu()
        } else {
            togglePopover()
        }
    }

    private func togglePopover() {
        guard let button = statusItem?.button else { return }
        if popover.isShown {
            popover.performClose(nil)
        } else {
            popover.show(relativeTo: button.bounds, of: button, preferredEdge: .minY)
        }
    }

    private func showMenu() {
        guard let button = statusItem?.button else { return }
        if popover.isShown { popover.performClose(nil) }
        let menu = NSMenu()
        func add(_ title: String, _ selector: Selector, key: String = "") {
            let item = NSMenuItem(title: title, action: selector, keyEquivalent: key)
            item.target = self
            menu.addItem(item)
        }
        add("Open Porthole", #selector(menuOpen))
        add("Refresh", #selector(menuRefresh), key: "r")
        menu.addItem(.separator())
        add("Check for Updates…", #selector(menuCheckUpdates))
        menu.addItem(.separator())
        add("Quit Porthole", #selector(menuQuit), key: "q")
        menu.popUp(positioning: nil,
                   at: NSPoint(x: 0, y: button.bounds.height + 5),
                   in: button)
    }

    @objc private func menuOpen() { togglePopover() }
    @objc private func menuRefresh() { coordinator.refresh() }
    @objc private func menuCheckUpdates() { UpdaterController.shared.checkForUpdates() }
    @objc private func menuQuit() { NSApp.terminate(nil) }

    func popoverDidShow(_ notification: Notification) {
        // Drop keyboard focus so no control draws a focus ring on open.
        popover.contentViewController?.view.window?.makeFirstResponder(nil)
        outsideClickMonitor = NSEvent.addGlobalMonitorForEvents(
            matching: [.leftMouseDown, .rightMouseDown]
        ) { [weak self] _ in
            self?.popover.performClose(nil)
        }
        coordinator.menuOpened()
    }

    func popoverDidClose(_ notification: Notification) {
        if let monitor = outsideClickMonitor {
            NSEvent.removeMonitor(monitor)
            outsideClickMonitor = nil
        }
        coordinator.menuClosed()
    }
}
