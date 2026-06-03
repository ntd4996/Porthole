import Foundation

public enum PortAssembler {
    public static func assemble(
        listens: [RawListen],
        projects: [Int32: ProjectInfo],
        displayNames: [Int32: String],
        tunnels: [TunnelInfo]
    ) -> [PortInfo] {
        let tunnelsByPort = Dictionary(grouping: tunnels, by: \.targetPort)
        let ports = listens.map { listen in
            PortInfo(
                port: listen.port,
                pid: listen.pid,
                command: listen.command,
                displayName: displayNames[listen.pid] ?? listen.command,
                project: projects[listen.pid],
                tunnels: tunnelsByPort[listen.port] ?? [])
        }
        return ports.sorted { a, b in
            let an = a.project?.name, bn = b.project?.name
            switch (an, bn) {
            case let (x?, y?) where x != y: return x < y
            case (nil, _?): return false  // Other after named projects
            case (_?, nil): return true
            default: return a.port < b.port
            }
        }
    }
}
