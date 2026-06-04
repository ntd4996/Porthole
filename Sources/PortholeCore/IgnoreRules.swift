import Foundation

/// Rules deciding which ports to hide from the main list. A port is ignored when
/// its number is in `ports`, or its process command/display name matches an entry
/// in `processes` (case-insensitive).
public struct IgnoreRules: Equatable, Sendable {
    public var processes: Set<String>
    public var ports: Set<Int>

    public init(processes: Set<String> = [], ports: Set<Int> = []) {
        self.processes = processes
        self.ports = ports
    }

    public func isIgnored(_ port: PortInfo) -> Bool {
        if ports.contains(port.port) { return true }
        return processes.contains { name in
            name.caseInsensitiveCompare(port.command) == .orderedSame
                || name.caseInsensitiveCompare(port.displayName) == .orderedSame
        }
    }
}
