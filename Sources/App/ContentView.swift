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
    private var ignoredPorts: [PortInfo] {
        state.ports.filter { ignore.rules.isIgnored($0) }
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
            .padding(.horizontal, 12).padding(.top, 10).padding(.bottom, 8)

            Picker("", selection: $tab) {
                Text("Ports").tag(Tab.ports)
                Text("Ignored").tag(Tab.ignored)
            }
            .pickerStyle(.segmented)
            .labelsHidden()
            .padding(.horizontal, 12).padding(.bottom, 8)

            Divider()

            ScrollView {
                Group {
                    switch tab {
                    case .ports: portsTab
                    case .ignored:
                        IgnoredTabView(ignore: ignore, ignoredPorts: ignoredPorts, onRefresh: onRefresh)
                    }
                }
                .padding(12)
            }
            .frame(maxHeight: 420)

            Divider()
            FooterView(portCount: visiblePorts.count, tunnelCount: tunnelCount)
        }
        .frame(width: 340)
        .focusEffectDisabled()
    }

    @ViewBuilder private var portsTab: some View {
        if visiblePorts.isEmpty {
            emptyState("Không có dev port nào đang chạy")
        } else {
            PortListView(ports: visiblePorts, mode: .normal, ignore: ignore, onRefresh: onRefresh)
        }
    }

    private func emptyState(_ text: String) -> some View {
        Text(text)
            .foregroundStyle(.secondary)
            .frame(maxWidth: .infinity, alignment: .center)
            .padding(.vertical, 24)
    }
}
