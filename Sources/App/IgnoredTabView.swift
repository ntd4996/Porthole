import SwiftUI
import PortholeCore

struct IgnoredTabView: View {
    let ignore: IgnoreStore
    let ignoredPorts: [PortInfo]
    var onRefresh: () -> Void
    @State private var newPort = ""

    /// Process rules with no currently-running port (so they don't appear as cards above).
    private var uncoveredProcesses: [String] {
        ignore.rules.processes.filter { name in
            !ignoredPorts.contains { p in
                name.caseInsensitiveCompare(p.command) == .orderedSame
                    || name.caseInsensitiveCompare(p.displayName) == .orderedSame
            }
        }.sorted()
    }
    private var uncoveredPorts: [Int] {
        ignore.rules.ports.filter { port in
            !ignoredPorts.contains { $0.port == port }
        }.sorted()
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            if !ignoredPorts.isEmpty {
                PortListView(ports: ignoredPorts, mode: .ignored, ignore: ignore, onRefresh: onRefresh)
            }

            if !uncoveredProcesses.isEmpty || !uncoveredPorts.isEmpty {
                rulesCard
            }

            if ignoredPorts.isEmpty && uncoveredProcesses.isEmpty && uncoveredPorts.isEmpty {
                Text("Nothing ignored")
                    .foregroundStyle(.secondary)
                    .frame(maxWidth: .infinity, alignment: .center)
                    .padding(.vertical, 24)
            }

            addPortField
        }
    }

    private var rulesCard: some View {
        VStack(alignment: .leading, spacing: 0) {
            Text("Rules (not running)")
                .font(.subheadline.weight(.semibold))
                .padding(.horizontal, 10).padding(.top, 8).padding(.bottom, 2)

            ForEach(uncoveredProcesses, id: \.self) { name in
                ruleRow(label: name, mono: false) { ignore.unignoreProcess(name) }
            }
            ForEach(uncoveredPorts, id: \.self) { port in
                ruleRow(label: ":\(port)", mono: true) { ignore.unignorePort(port) }
            }
        }
        .padding(.bottom, 4)
        .background(RoundedRectangle(cornerRadius: 10).fill(Color.primary.opacity(0.05)))
        .overlay(RoundedRectangle(cornerRadius: 10).strokeBorder(Color.primary.opacity(0.08)))
    }

    private func ruleRow(label: String, mono: Bool, remove: @escaping () -> Void) -> some View {
        HStack {
            Text(label)
                .font(mono ? .system(.callout, design: .monospaced) : .body)
                .lineLimit(1)
            Spacer()
            Button(action: remove) { Image(systemName: "eye") }
                .buttonStyle(IconButtonStyle()).help("Un-ignore")
        }
        .padding(.horizontal, 10).padding(.vertical, 5)
    }

    private var addPortField: some View {
        HStack {
            TextField("Ignore a port…", text: $newPort)
                .textFieldStyle(.roundedBorder)
                .onSubmit(addPort)
            Button("Add", action: addPort).disabled(Int(newPort) == nil)
        }
    }

    private func addPort() {
        guard let port = Int(newPort) else { return }
        ignore.ignorePort(port)
        newPort = ""
    }
}
