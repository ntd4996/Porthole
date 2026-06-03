import SwiftUI

struct FooterView: View {
    let portCount: Int
    let tunnelCount: Int

    var body: some View {
        HStack {
            Text("\(portCount) ports · \(tunnelCount) tunnels")
                .font(.caption).foregroundStyle(.secondary)
            Spacer()
            Button("Quit") { NSApplication.shared.terminate(nil) }
                .buttonStyle(.borderless)
        }
        .padding(.horizontal, 12).padding(.vertical, 8)
    }
}
