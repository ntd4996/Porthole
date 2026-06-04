import SwiftUI
import PortholeCore

struct ProjectGroupView: View {
    let title: String
    let kind: ProjectKind?
    let ports: [PortInfo]
    let mode: PortRowMode
    let ignore: IgnoreStore
    var onRefresh: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            HStack(spacing: 6) {
                Text(title).font(.subheadline.weight(.semibold))
                if let kind, kind != .unknown {
                    Text(kind.rawValue)
                        .font(.caption2)
                        .padding(.horizontal, 6).padding(.vertical, 1)
                        .background(Capsule().fill(Color.secondary.opacity(0.22)))
                        .foregroundStyle(.secondary)
                }
                Spacer()
            }
            .padding(.horizontal, 10).padding(.top, 8).padding(.bottom, 2)

            ForEach(Array(ports.enumerated()), id: \.element.id) { idx, port in
                if idx > 0 { Divider().padding(.leading, 10) }
                PortRowView(port: port, mode: mode, ignore: ignore, onRefresh: onRefresh)
            }
        }
        .padding(.bottom, 4)
        .background(RoundedRectangle(cornerRadius: 10).fill(Color.primary.opacity(0.05)))
        .overlay(RoundedRectangle(cornerRadius: 10).strokeBorder(Color.primary.opacity(0.08)))
    }
}
