import Foundation

public enum NgrokParser {
    public static func parse(_ data: Data) -> [TunnelInfo] {
        guard let obj = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
              let tunnels = obj["tunnels"] as? [[String: Any]] else { return [] }
        return tunnels.compactMap { tunnel in
            guard let config = tunnel["config"] as? [String: Any],
                  let addr = config["addr"] as? String,
                  let port = port(fromAddr: addr) else { return nil }
            return TunnelInfo(
                provider: .ngrok,
                publicURL: tunnel["public_url"] as? String,
                targetPort: port)
        }
    }

    private static func port(fromAddr addr: String) -> Int? {
        guard let colon = addr.lastIndex(of: ":") else { return Int(addr) } // bare port, e.g. "3000"
        return Int(addr[addr.index(after: colon)...])
    }
}
