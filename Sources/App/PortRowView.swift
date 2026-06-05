import SwiftUI
import PortholeCore

enum PortRowMode { case normal, ignored }

struct PortRowView: View {
    let port: PortInfo
    let mode: PortRowMode
    let ignore: IgnoreStore
    var onRefresh: () -> Void
    @State private var confirmingKill = false

    var body: some View {
        VStack(alignment: .leading, spacing: 3) {
            HStack(spacing: 8) {
                Text(verbatim: ":\(port.port)")
                    .font(.system(.callout, design: .monospaced)).bold()
                    .foregroundStyle(.tint)
                    .frame(width: 60, alignment: .leading)
                Text(port.displayName).foregroundStyle(.secondary).lineLimit(1)
                Spacer(minLength: 4)
                actions
            }
            ForEach(port.tunnels, id: \.targetPort) { tunnel in
                tunnelLabel(tunnel)
            }
        }
        .padding(.horizontal, 10).padding(.vertical, 5)
        .contentShape(Rectangle())
        .contextMenu {
            switch mode {
            case .normal:
                Button("Ignore process \(port.command)") { ignore.ignoreProcess(port.command) }
                Button("Ignore port \(port.port)") { ignore.ignorePort(port.port) }
            case .ignored:
                Button("Un-ignore") { ignore.unignoreMatching(port) }
            }
        }
        .confirmationDialog("Kill PID \(port.pid)?", isPresented: $confirmingKill) {
            Button("Kill", role: .destructive) {
                Actions.kill(pid: port.pid)
                onRefresh()
            }
        }
    }

    @ViewBuilder private var actions: some View {
        iconButton("arrow.up.right.square", "Open in browser") {
            Actions.openInBrowser(port: port.port)
        }
        iconButton("doc.on.doc", "Copy URL") {
            Actions.copy("http://localhost:\(port.port)")
        }
        switch mode {
        case .normal:
            iconButton("eye.slash", "Ignore \(port.command)") {
                ignore.ignoreProcess(port.command)
            }
            iconButton("xmark.circle", "Kill process", color: .red) {
                confirmingKill = true
            }
        case .ignored:
            iconButton("eye", "Un-ignore") {
                ignore.unignoreMatching(port)
            }
        }
    }

    private func iconButton(
        _ name: String, _ help: String,
        color: Color = .secondary, action: @escaping () -> Void
    ) -> some View {
        Button(action: action) {
            Image(systemName: name)
        }
        .buttonStyle(IconButtonStyle(color: color))
        .help(help)
    }

    @ViewBuilder private func tunnelLabel(_ tunnel: TunnelInfo) -> some View {
        let color = Self.providerColor(tunnel.provider)
        Group {
            if let url = tunnel.publicURL {
                Button { Actions.openURL(url) } label: {
                    pill(tunnel.provider.rawValue, detail: url, color: color)
                }
                .buttonStyle(.plain)
            } else {
                pill(tunnel.provider.rawValue, detail: ":\(tunnel.targetPort)", color: color)
            }
        }
        .padding(.leading, 60)
    }

    private func pill(_ name: String, detail: String, color: Color) -> some View {
        HStack(spacing: 4) {
            Image(systemName: "globe").font(.caption2)
            Text(name).font(.caption2.weight(.semibold))
            Text(detail).font(.caption2).lineLimit(1)
        }
        .padding(.horizontal, 7).padding(.vertical, 2)
        .background(Capsule().fill(color.opacity(0.18)))
        .foregroundStyle(color)
    }

    static func providerColor(_ p: TunnelProvider) -> Color {
        switch p {
        case .cloudflare: return .orange
        case .ngrok: return .green
        case .tailscale: return .blue
        case .localtunnel: return .purple
        }
    }
}
