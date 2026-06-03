import Foundation

public enum LocaltunnelParser {
    public static func parse(fromCommandLine cmd: String) -> TunnelInfo? {
        let tokens = cmd.split(separator: " ").map(String.init)
        func flag(_ name: String) -> String? {
            guard let i = tokens.firstIndex(of: name), i + 1 < tokens.count else { return nil }
            return tokens[i + 1]
        }
        guard let portStr = flag("--port") ?? flag("-p"), let port = Int(portStr) else { return nil }
        let url = flag("--subdomain").map { "https://\($0).loca.lt" }
        return TunnelInfo(provider: .localtunnel, publicURL: url, targetPort: port)
    }
}
