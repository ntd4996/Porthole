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
    private let seedVersionKey = "porthole.ignore.seedVersion"

    /// Bump when new entries are added to `defaultProcesses` so existing installs
    /// pick them up once (without clobbering the user's own removals afterwards).
    private static let seedVersion = 2

    /// Noisy macOS system services / apps that hold ports but are never dev servers.
    static let defaultProcesses = [
        "ControlCenter", "rapportd", "sharingd", "AirPlayXPCHelper",
        "remoted", "identityservicesd",
        "Raycast", "Cursor Helper (Plugin)", "Discord Helper (Renderer)",
    ]

    init() {
        if defaults.object(forKey: processesKey) == nil,
           defaults.object(forKey: portsKey) == nil {
            rules = IgnoreRules(processes: Set(Self.defaultProcesses), ports: [])
            defaults.set(Self.seedVersion, forKey: seedVersionKey)
            persist()
        } else {
            let procs = defaults.stringArray(forKey: processesKey) ?? []
            let ports = (defaults.array(forKey: portsKey) as? [Int]) ?? []
            rules = IgnoreRules(processes: Set(procs), ports: Set(ports))
            migrateSeedIfNeeded()
        }
    }

    /// One-time merge of newly-added default processes for older installs.
    private func migrateSeedIfNeeded() {
        let stored = defaults.integer(forKey: seedVersionKey)
        guard stored < Self.seedVersion else { return }
        rules.processes.formUnion(Self.defaultProcesses)
        defaults.set(Self.seedVersion, forKey: seedVersionKey)
        persist()
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

    /// Removes whichever rule (port number or process name) currently hides this port.
    func unignoreMatching(_ port: PortInfo) {
        rules.ports.remove(port.port)
        for name in rules.processes where
            name.caseInsensitiveCompare(port.command) == .orderedSame
            || name.caseInsensitiveCompare(port.displayName) == .orderedSame {
            rules.processes.remove(name)
        }
        persist()
    }

    private func persist() {
        defaults.set(Array(rules.processes), forKey: processesKey)
        defaults.set(Array(rules.ports), forKey: portsKey)
    }
}
