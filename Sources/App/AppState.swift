import Foundation
import Observation
import PortholeCore

@MainActor
@Observable
final class AppState {
    var ports: [PortInfo] = []
    var lastError: String?
    var isScanning = true   // true until the first scan completes (shows loading, not empty)
    var didScan = false     // a scan has finished at least once

    var tunnelCount: Int { ports.reduce(0) { $0 + $1.tunnels.count } }
}
