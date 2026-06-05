import SwiftUI

struct FooterView: View {
    let portCount: Int
    let tunnelCount: Int

    var body: some View {
        HStack(spacing: 12) {
            Text("\(portCount) ports · \(tunnelCount) tunnels")
                .font(.caption).foregroundStyle(.secondary)
            Spacer(minLength: 8)
            Button("Check for Updates") { UpdaterController.shared.checkForUpdates() }
                .buttonStyle(HoverTextButtonStyle())
                .help("Check for a newer version")
            Button("Quit") { NSApplication.shared.terminate(nil) }
                .buttonStyle(HoverTextButtonStyle())
        }
        .padding(.horizontal, 12).padding(.vertical, 8)
    }
}
