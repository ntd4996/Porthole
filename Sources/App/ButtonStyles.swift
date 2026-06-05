import SwiftUI

/// Icon button with a hover background and a press animation, so the icons read
/// as clickable and a tap visibly registers. Pair with `.help(...)` for a tooltip.
struct IconButtonStyle: ButtonStyle {
    var color: Color = .secondary

    func makeBody(configuration: Configuration) -> some View {
        StyleBody(configuration: configuration, color: color)
    }

    private struct StyleBody: View {
        let configuration: ButtonStyleConfiguration
        let color: Color
        @State private var hovering = false

        var body: some View {
            configuration.label
                .foregroundStyle(color)
                .frame(width: 26, height: 24)
                .background(
                    RoundedRectangle(cornerRadius: 6, style: .continuous)
                        .fill(Color.primary.opacity(configuration.isPressed ? 0.20 : (hovering ? 0.10 : 0)))
                )
                .scaleEffect(configuration.isPressed ? 0.82 : 1)
                .animation(.easeOut(duration: 0.10), value: hovering)
                .animation(.spring(response: 0.22, dampingFraction: 0.5), value: configuration.isPressed)
                .contentShape(Rectangle())
                .onHover { hovering = $0 }
        }
    }
}

/// Small text button (footer) that brightens on hover and dims on press.
struct HoverTextButtonStyle: ButtonStyle {
    func makeBody(configuration: Configuration) -> some View {
        StyleBody(configuration: configuration)
    }

    private struct StyleBody: View {
        let configuration: ButtonStyleConfiguration
        @State private var hovering = false

        var body: some View {
            configuration.label
                .font(.caption)
                .foregroundStyle(hovering ? Color.primary : Color.secondary)
                .opacity(configuration.isPressed ? 0.55 : 1)
                .animation(.easeOut(duration: 0.10), value: hovering)
                .contentShape(Rectangle())
                .onHover { hovering = $0 }
        }
    }
}
