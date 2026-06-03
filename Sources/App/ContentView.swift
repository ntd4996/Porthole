import SwiftUI
import PortholeCore

struct ContentView: View {
    let state: AppState
    var onRefresh: () -> Void

    private var grouped: [(title: String, kind: ProjectKind?, ports: [PortInfo])] {
        var named: [String: (ProjectKind, [PortInfo])] = [:]
        var other: [PortInfo] = []
        for port in state.ports {
            if let project = port.project {
                named[project.name, default: (project.kind, [])].1.append(port)
            } else {
                other.append(port)
            }
        }
        var groups = named.keys.sorted().map { key in
            (title: key, kind: Optional(named[key]!.0), ports: named[key]!.1)
        }
        if !other.isEmpty { groups.append((title: "Other", kind: nil, ports: other)) }
        return groups
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            HStack {
                Text("Porthole").font(.headline)
                Spacer()
                Button { onRefresh() } label: { Image(systemName: "arrow.clockwise") }
                    .buttonStyle(.borderless).help("Refresh")
            }
            .padding(.horizontal, 12).padding(.top, 10).padding(.bottom, 4)

            Divider()

            if state.ports.isEmpty {
                Text("Không có dev port nào đang chạy")
                    .foregroundStyle(.secondary)
                    .frame(maxWidth: .infinity, alignment: .center)
                    .padding(.vertical, 24)
            } else {
                ScrollView {
                    VStack(alignment: .leading, spacing: 4) {
                        ForEach(grouped, id: \.title) { group in
                            ProjectGroupView(
                                title: group.title, kind: group.kind,
                                ports: group.ports, onRefresh: onRefresh)
                        }
                    }
                    .padding(.horizontal, 12)
                }
                .frame(maxHeight: 400)
            }

            Divider()
            FooterView(portCount: state.ports.count, tunnelCount: state.tunnelCount)
        }
        .frame(width: 320)
    }
}
