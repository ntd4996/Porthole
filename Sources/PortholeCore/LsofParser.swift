import Foundation

public enum LsofParser {
    /// Parse `lsof -nP -iTCP -sTCP:LISTEN -F pcn` output into unique LISTEN sockets.
    public static func parseListens(_ output: String) -> [RawListen] {
        var currentPID: Int32?
        var currentCommand = ""
        var seen = Set<String>()
        var result: [RawListen] = []

        for line in output.split(separator: "\n", omittingEmptySubsequences: true) {
            guard let tag = line.first else { continue }
            let value = String(line.dropFirst())
            switch tag {
            case "p":
                currentPID = Int32(value)
                currentCommand = ""
            case "c":
                currentCommand = value
            case "n":
                guard let pid = currentPID, let port = port(from: value) else { continue }
                let key = "\(pid)-\(port)"
                if seen.insert(key).inserted {
                    result.append(RawListen(pid: pid, command: currentCommand, port: port))
                }
            default:
                continue
            }
        }
        return result
    }

    /// Extract the port from an lsof address such as `*:6379`, `127.0.0.1:3000`, `[::1]:3000`.
    private static func port(from address: String) -> Int? {
        guard let colon = address.lastIndex(of: ":") else { return nil }
        let portPart = address[address.index(after: colon)...]
        return Int(portPart)
    }
}
