import SwiftUI
import PortholeCore

struct ProjectGroupView: View {
    let title: String
    let kind: ProjectKind?
    let ports: [PortInfo]
    var onRefresh: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            HStack(spacing: 6) {
                Text(title).font(.headline)
                if let kind, kind != .unknown {
                    Text("(\(kind.rawValue))").font(.caption).foregroundStyle(.secondary)
                }
            }
            .padding(.top, 6)
            ForEach(ports) { port in
                PortRowView(port: port, onRefresh: onRefresh)
            }
        }
    }
}
