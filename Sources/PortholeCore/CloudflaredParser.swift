import Foundation

public enum CloudflaredParser {
    /// Extract the target port from a `--url <addr>` flag, if present.
    public static func targetPort(fromCommandLine cmd: String) -> Int? {
        let tokens = cmd.split(separator: " ").map(String.init)
        guard let i = tokens.firstIndex(of: "--url"), i + 1 < tokens.count else { return nil }
        return port(fromURLLike: tokens[i + 1])
    }

    /// Parse named-tunnel ingress rules (hostname -> localhost:port) from config.yml.
    public static func ingress(fromConfigYAML yaml: String) -> [TunnelInfo] {
        var result: [TunnelInfo] = []
        var pendingHost: String?
        for raw in yaml.split(separator: "\n", omittingEmptySubsequences: true) {
            let line = raw.trimmingCharacters(in: .whitespaces)
            if let host = value(line, key: "- hostname") ?? value(line, key: "hostname") {
                pendingHost = host
            } else if let service = value(line, key: "service") {
                if let host = pendingHost, let port = port(fromURLLike: service) {
                    result.append(TunnelInfo(
                        provider: .cloudflare,
                        publicURL: "https://\(host)",
                        targetPort: port))
                }
                pendingHost = nil
            }
        }
        return result
    }

    private static func value(_ line: String, key: String) -> String? {
        guard line.hasPrefix(key) else { return nil }
        let rest = line.dropFirst(key.count)
        guard rest.first == ":" else { return nil }
        let v = rest.dropFirst().trimmingCharacters(in: .whitespaces)
        return v.isEmpty ? nil : v
    }

    private static func port(fromURLLike s: String) -> Int? {
        guard let colon = s.lastIndex(of: ":") else { return nil }
        let tail = s[s.index(after: colon)...].prefix { $0.isNumber }
        return Int(tail)
    }
}
