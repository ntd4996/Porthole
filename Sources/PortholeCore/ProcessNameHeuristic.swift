import Foundation

public enum ProcessNameHeuristic {
    /// Ordered (needle, label). First needle found in the full command line wins.
    private static let rules: [(String, String)] = [
        ("vite", "vite"),
        ("next-server", "next"),
        ("next dev", "next"),
        ("nuxt", "nuxt"),
        ("astro", "astro"),
        ("webpack", "webpack"),
        ("remix", "remix"),
        ("uvicorn", "uvicorn"),
        ("gunicorn", "gunicorn"),
        ("flask", "flask"),
        ("manage.py runserver", "django"),
        ("rails", "rails"),
        ("prisma", "prisma"),
        ("storybook", "storybook"),
    ]

    public static func displayName(command: String, fullCommand: String) -> String {
        let haystack = fullCommand.lowercased()
        for (needle, label) in rules where haystack.contains(needle) {
            return label
        }
        return command
    }
}
