import Foundation
import Observation
import PortholeCore

/// Persists the ignore list (UserDefaults) and exposes it as observable
/// `IgnoreRules`. Seeds common macOS system services on first launch.
@MainActor
@Observable
final class IgnoreStore {
    private(set) var rules: IgnoreRules

    private let defaults = UserDefaults.standard
    private let processesKey = "porthole.ignore.processes"
    private let portsKey = "porthole.ignore.ports"

    /// Noisy macOS system services that hold ports but are never dev servers.
    static let defaultProcesses = [
        "ControlCenter", "rapportd", "sharingd", "AirPlayXPCHelper",
        "remoted", "identityservicesd",
    ]

    init() {
        if defaults.object(forKey: processesKey) == nil,
           defaults.object(forKey: portsKey) == nil {
            rules = IgnoreRules(processes: Set(Self.defaultProcesses), ports: [])
            persist()
        } else {
            let procs = defaults.stringArray(forKey: processesKey) ?? []
            let ports = (defaults.array(forKey: portsKey) as? [Int]) ?? []
            rules = IgnoreRules(processes: Set(procs), ports: Set(ports))
        }
    }

    func ignoreProcess(_ name: String) {
        guard !name.isEmpty else { return }
        rules.processes.insert(name)
        persist()
    }

    func ignorePort(_ port: Int) {
        rules.ports.insert(port)
        persist()
    }

    func unignoreProcess(_ name: String) {
        rules.processes.remove(name)
        persist()
    }

    func unignorePort(_ port: Int) {
        rules.ports.remove(port)
        persist()
    }

    private func persist() {
        defaults.set(Array(rules.processes), forKey: processesKey)
        defaults.set(Array(rules.ports), forKey: portsKey)
    }
}
