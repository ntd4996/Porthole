import SwiftUI

@MainActor
@main
struct PortholeApp: App {
    @State private var state: AppState
    @State private var coordinator: ScanCoordinator

    init() {
        let s = AppState()
        _state = State(initialValue: s)
        _coordinator = State(initialValue: ScanCoordinator(state: s))
    }

    var body: some Scene {
        MenuBarExtra("Porthole", systemImage: "circle.circle") {
            ContentView(state: state) { coordinator.refresh() }
                .onAppear { coordinator.menuOpened() }
                .onDisappear { coordinator.menuClosed() }
        }
        .menuBarExtraStyle(.window)
    }
}
