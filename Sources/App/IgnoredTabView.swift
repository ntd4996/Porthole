import SwiftUI

struct IgnoredTabView: View {
    let ignore: IgnoreStore
    @State private var newPort = ""

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 4) {
                if ignore.rules.processes.isEmpty && ignore.rules.ports.isEmpty {
                    Text("Chưa ignore gì")
                        .foregroundStyle(.secondary)
                        .frame(maxWidth: .infinity, alignment: .center)
                        .padding(.vertical, 24)
                }

                if !ignore.rules.processes.isEmpty {
                    Text("Processes").font(.headline).padding(.top, 6)
                    ForEach(ignore.rules.processes.sorted(), id: \.self) { name in
                        HStack {
                            Text(name).lineLimit(1)
                            Spacer()
                            Button { ignore.unignoreProcess(name) } label: {
                                Image(systemName: "arrow.uturn.backward")
                            }.buttonStyle(.plain).help("Un-ignore")
                        }
                        .padding(.vertical, 2)
                    }
                }

                if !ignore.rules.ports.isEmpty {
                    Text("Ports").font(.headline).padding(.top, 6)
                    ForEach(ignore.rules.ports.sorted(), id: \.self) { port in
                        HStack {
                            Text(verbatim: ":\(port)").font(.system(.body, design: .monospaced))
                            Spacer()
                            Button { ignore.unignorePort(port) } label: {
                                Image(systemName: "arrow.uturn.backward")
                            }.buttonStyle(.plain).help("Un-ignore")
                        }
                        .padding(.vertical, 2)
                    }
                }

                Divider().padding(.vertical, 6)
                HStack {
                    TextField("Ignore a port…", text: $newPort)
                        .textFieldStyle(.roundedBorder)
                        .onSubmit(addPort)
                    Button("Add", action: addPort)
                        .disabled(Int(newPort) == nil)
                }
            }
            .padding(.horizontal, 12)
        }
    }

    private func addPort() {
        guard let port = Int(newPort) else { return }
        ignore.ignorePort(port)
        newPort = ""
    }
}
