import SwiftUI

struct FooterView: View {
    let portCount: Int
    let tunnelCount: Int

    var body: some View {
        HStack(spacing: 12) {
            Text(loc("\(portCount) ports · \(tunnelCount) tunnels"))
                .font(.caption).foregroundStyle(.secondary)
            Spacer(minLength: 8)
            Button(loc("Check for Updates")) { UpdaterController.shared.checkForUpdates() }
                .buttonStyle(HoverTextButtonStyle())
                .help(loc("Check for a newer version"))
            Button(loc("Quit")) { NSApplication.shared.terminate(nil) }
                .buttonStyle(HoverTextButtonStyle())
        }
        .padding(.horizontal, 12).padding(.vertical, 8)
    }
}
