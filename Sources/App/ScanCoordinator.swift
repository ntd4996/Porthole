import Foundation
import PortholeCore

@MainActor
final class ScanCoordinator {
    private let state: AppState
    private let service: ScanService
    private var timer: Timer?

    init(state: AppState, service: ScanService = ScanService()) {
        self.state = state
        self.service = service
    }

    func refresh() {
        state.isScanning = true
        Task { await performScan() }
    }

    func menuOpened() {
        refresh()
        timer?.invalidate()
        timer = Timer.scheduledTimer(withTimeInterval: 4.0, repeats: true) { [weak self] _ in
            Task { @MainActor in self?.refresh() }
        }
    }

    func menuClosed() {
        timer?.invalidate()
        timer = nil
    }

    private func performScan() async {
        state.isScanning = true
        let ports = await service.scan()
        state.ports = ports
        state.isScanning = false
        state.didScan = true
    }
}
