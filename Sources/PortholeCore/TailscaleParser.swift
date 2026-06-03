import Foundation

public enum TailscaleParser {
    public static func parse(_ output: String) -> [TunnelInfo] {
        var result: [TunnelInfo] = []
        var currentHost: String?
        for raw in output.split(separator: "\n", omittingEmptySubsequences: true) {
            let line = raw.trimmingCharacters(in: .whitespaces)
            if line.hasPrefix("https://") {
                currentHost = line.split(separator: " ").first.map(String.init)
            } else if line.contains("proxy "), let host = currentHost,
                      let port = proxyPort(line) {
                result.append(TunnelInfo(provider: .tailscale, publicURL: host, targetPort: port))
            }
        }
        return result
    }

    private static func proxyPort(_ line: String) -> Int? {
        guard let range = line.range(of: "proxy ") else { return nil }
        let target = line[range.upperBound...]
        guard let colon = target.lastIndex(of: ":") else { return nil }
        let tail = target[target.index(after: colon)...].prefix { $0.isNumber }
        return Int(tail)
    }
}
