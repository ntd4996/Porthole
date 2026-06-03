import Foundation

public enum ProjectKind: String, Sendable, Equatable {
    case node, python, go, ruby, rust, unknown
}

public enum TunnelProvider: String, Sendable, Equatable {
    case cloudflare, ngrok, tailscale, localtunnel
}

public struct ProjectInfo: Equatable, Sendable {
    public let path: String
    public let name: String
    public let kind: ProjectKind
    public init(path: String, name: String, kind: ProjectKind) {
        self.path = path; self.name = name; self.kind = kind
    }
}

public struct TunnelInfo: Equatable, Sendable {
    public let provider: TunnelProvider
    public let publicURL: String?
    public let targetPort: Int
    public init(provider: TunnelProvider, publicURL: String?, targetPort: Int) {
        self.provider = provider; self.publicURL = publicURL; self.targetPort = targetPort
    }
}

/// One LISTEN socket as parsed from lsof, before enrichment.
public struct RawListen: Equatable, Sendable {
    public let pid: Int32
    public let command: String
    public let port: Int
    public init(pid: Int32, command: String, port: Int) {
        self.pid = pid; self.command = command; self.port = port
    }
}

public struct PortInfo: Equatable, Sendable, Identifiable {
    public let port: Int
    public let pid: Int32
    public let command: String
    public let displayName: String
    public var project: ProjectInfo?
    public var tunnels: [TunnelInfo]
    public init(port: Int, pid: Int32, command: String, displayName: String,
                project: ProjectInfo? = nil, tunnels: [TunnelInfo] = []) {
        self.port = port; self.pid = pid; self.command = command
        self.displayName = displayName; self.project = project; self.tunnels = tunnels
    }
    public var id: String { "\(pid)-\(port)" }
}
