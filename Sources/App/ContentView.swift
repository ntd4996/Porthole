import SwiftUI
import PortholeCore

struct ContentView: View {
    let state: AppState
    let ignore: IgnoreStore
    var onRefresh: () -> Void

    enum Tab: Hashable { case ports, ignored }
    @State private var tab: Tab = .ports

    private var visiblePorts: [PortInfo] {
        state.ports.filter { !ignore.rules.isIgnored($0) }
    }

    private var grouped: [(title: String, kind: ProjectKind?, ports: [PortInfo])] {
        var named: [String: (ProjectKind, [PortInfo])] = [:]
        var other: [PortInfo] = []
        for port in visiblePorts {
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

    private var tunnelCount: Int {
        visiblePorts.reduce(0) { $0 + $1.tunnels.count }
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            HStack {
                Text("Porthole").font(.headline)
                Spacer()
                Button { onRefresh() } label: { Image(systemName: "arrow.clockwise") }
                    .buttonStyle(.plain).help("Refresh")
            }
            .padding(.horizontal, 12).padding(.top, 10).padding(.bottom, 6)

            Picker("", selection: $tab) {
                Text("Ports").tag(Tab.ports)
                Text("Ignored").tag(Tab.ignored)
            }
            .pickerStyle(.segmented)
            .labelsHidden()
            .padding(.horizontal, 12).padding(.bottom, 6)

            Divider()

            switch tab {
            case .ports: portsTab
            case .ignored:
                IgnoredTabView(ignore: ignore)
                    .frame(maxHeight: 400)
            }

            Divider()
            FooterView(portCount: visiblePorts.count, tunnelCount: tunnelCount)
        }
        .frame(width: 320)
        .focusEffectDisabled()
    }

    @ViewBuilder private var portsTab: some View {
        if visiblePorts.isEmpty {
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
                            ports: group.ports, ignore: ignore, onRefresh: onRefresh)
                    }
                }
                .padding(.horizontal, 12)
            }
            .frame(maxHeight: 400)
        }
    }
}
