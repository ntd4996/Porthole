import Foundation
import Observation
import PortholeCore

@MainActor
@Observable
final class AppState {
    var ports: [PortInfo] = []
    var lastError: String?
    var isScanning = false

    var tunnelCount: Int { ports.reduce(0) { $0 + $1.tunnels.count } }
}
