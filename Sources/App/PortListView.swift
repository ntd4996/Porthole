import SwiftUI
import PortholeCore

/// Groups ports by project (unprojected ports under "Other") and renders one card
/// per group. Shared by the Ports and Ignored tabs.
struct PortListView: View {
    let ports: [PortInfo]
    let mode: PortRowMode
    let ignore: IgnoreStore
    var onRefresh: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            ForEach(Self.group(ports), id: \.title) { group in
                ProjectGroupView(
                    title: group.title, kind: group.kind, ports: group.ports,
                    mode: mode, ignore: ignore, onRefresh: onRefresh)
            }
        }
    }

    static func group(_ ports: [PortInfo]) -> [(title: String, kind: ProjectKind?, ports: [PortInfo])] {
        var named: [String: (ProjectKind, [PortInfo])] = [:]
        var other: [PortInfo] = []
        for port in ports {
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
}
