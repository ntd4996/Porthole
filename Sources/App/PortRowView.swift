import SwiftUI
import PortholeCore

struct PortRowView: View {
    let port: PortInfo
    var onRefresh: () -> Void
    @State private var confirmingKill = false

    var body: some View {
        VStack(alignment: .leading, spacing: 2) {
            HStack(spacing: 8) {
                Text(verbatim: ":\(port.port)")
                    .font(.system(.body, design: .monospaced)).bold()
                    .frame(width: 64, alignment: .leading)
                Text(port.displayName).foregroundStyle(.secondary).lineLimit(1)
                Spacer()
                if !port.tunnels.isEmpty {
                    Image(systemName: "globe").foregroundStyle(.blue)
                }
                Button { Actions.openInBrowser(port: port.port) } label: {
                    Image(systemName: "arrow.up.right.square")
                }.buttonStyle(.plain).help("Open in browser")
                Button { Actions.copy("http://localhost:\(port.port)") } label: {
                    Image(systemName: "doc.on.doc")
                }.buttonStyle(.plain).help("Copy URL")
                Button { confirmingKill = true } label: {
                    Image(systemName: "xmark.circle").foregroundStyle(.red)
                }.buttonStyle(.plain).help("Kill process")
            }
            ForEach(port.tunnels, id: \.targetPort) { tunnel in
                if let url = tunnel.publicURL {
                    Button { Actions.openURL(url) } label: {
                        Text("\(tunnel.provider.rawValue) \(url)")
                            .font(.caption).foregroundStyle(.blue).lineLimit(1)
                    }.buttonStyle(.plain)
                } else {
                    Text("\(tunnel.provider.rawValue) -> :\(tunnel.targetPort)")
                        .font(.caption).foregroundStyle(.secondary)
                }
            }
        }
        .padding(.vertical, 2)
        .confirmationDialog("Kill PID \(port.pid)?", isPresented: $confirmingKill) {
            Button("Kill", role: .destructive) {
                Actions.kill(pid: port.pid)
                onRefresh()
            }
        }
    }
}
