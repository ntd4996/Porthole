import Foundation
import PortholeCore

struct ScanService {
    let runner: CommandRunner
    init(runner: CommandRunner = SystemCommandRunner()) { self.runner = runner }

    private let lsof = "/usr/sbin/lsof"
    private let ps = "/bin/ps"

    func scan() async -> [PortInfo] {
        let listens = await listListens()
        let pids = Set(listens.map(\.pid))

        var projects: [Int32: ProjectInfo] = [:]
        var names: [Int32: String] = [:]
        for pid in pids {
            if let cwd = await cwd(forPID: pid), let info = ProjectResolver.resolve(cwd: cwd) {
                projects[pid] = info
            }
            if let full = await fullCommand(forPID: pid) {
                let cmd = listens.first { $0.pid == pid }?.command ?? full
                names[pid] = ProcessNameHeuristic.displayName(command: cmd, fullCommand: full)
            }
        }

        let tunnels = await scanTunnels(listens: listens)
        return PortAssembler.assemble(
            listens: listens, projects: projects, displayNames: names, tunnels: tunnels)
    }

    private func listListens() async -> [RawListen] {
        guard let out = await runner.run(lsof, ["-nP", "-iTCP", "-sTCP:LISTEN", "-F", "pcn"])
        else { return [] }
        return LsofParser.parseListens(out)
    }

    private func cwd(forPID pid: Int32) async -> String? {
        guard let out = await runner.run(lsof, ["-a", "-p", "\(pid)", "-d", "cwd", "-F", "n"])
        else { return nil }
        return out.split(separator: "\n").first { $0.hasPrefix("n") }.map { String($0.dropFirst()) }
    }

    private func fullCommand(forPID pid: Int32) async -> String? {
        await runner.run(ps, ["-p", "\(pid)", "-o", "command="])?
            .trimmingCharacters(in: .whitespacesAndNewlines)
    }

    private func scanTunnels(listens: [RawListen]) async -> [TunnelInfo] {
        var tunnels: [TunnelInfo] = []
        if let ngrok = await fetchNgrok() { tunnels += ngrok }
        tunnels += await scanCloudflared()
        tunnels += cloudflaredConfigTunnels()
        var tailscaleOut = await runner.run("/usr/local/bin/tailscale", ["serve", "status"])
        if tailscaleOut == nil {
            tailscaleOut = await runner.run("/opt/homebrew/bin/tailscale", ["serve", "status"])
        }
        if let tail = tailscaleOut {
            tunnels += TailscaleParser.parse(tail)
        }
        return tunnels
    }

    private func fetchNgrok() async -> [TunnelInfo]? {
        var req = URLRequest(url: URL(string: "http://127.0.0.1:4040/api/tunnels")!)
        req.timeoutInterval = 0.5
        guard let (data, _) = try? await URLSession.shared.data(for: req) else { return nil }
        return NgrokParser.parse(data)
    }

    private func scanCloudflared() async -> [TunnelInfo] {
        guard let out = await runner.run(ps, ["-axo", "pid=,command="]) else { return [] }
        var result: [TunnelInfo] = []
        for line in out.split(separator: "\n") {
            let cmd = String(line)
            if cmd.contains("cloudflared"),
               let port = CloudflaredParser.targetPort(fromCommandLine: cmd) {
                result.append(TunnelInfo(provider: .cloudflare, publicURL: nil, targetPort: port))
            }
            if isLocaltunnel(cmd), let lt = LocaltunnelParser.parse(fromCommandLine: cmd) {
                result.append(lt)
            }
        }
        return result
    }

    /// True when the command line invokes the localtunnel CLI (`lt` token or a `.../lt` path).
    private func isLocaltunnel(_ cmd: String) -> Bool {
        cmd.split(separator: " ").contains { $0 == "lt" || $0.hasSuffix("/lt") }
    }

    private func cloudflaredConfigTunnels() -> [TunnelInfo] {
        let path = ("~/.cloudflared/config.yml" as NSString).expandingTildeInPath
        guard let yaml = try? String(contentsOfFile: path, encoding: .utf8) else { return [] }
        return CloudflaredParser.ingress(fromConfigYAML: yaml)
    }
}
