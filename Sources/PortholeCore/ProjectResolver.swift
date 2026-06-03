import Foundation

public enum ProjectResolver {
    public static func resolve(cwd: String, fileManager fm: FileManager = .default) -> ProjectInfo? {
        var dir = URL(fileURLWithPath: cwd, isDirectory: true).standardizedFileURL
        while true {
            if let info = marker(in: dir, fm: fm) { return info }
            let parent = dir.deletingLastPathComponent().standardizedFileURL
            if parent.path == dir.path { return nil } // reached root
            dir = parent
        }
    }

    private static func marker(in dir: URL, fm: FileManager) -> ProjectInfo? {
        func has(_ name: String) -> Bool {
            fm.fileExists(atPath: dir.appendingPathComponent(name).path)
        }
        if has("package.json") {
            let name = packageName(at: dir.appendingPathComponent("package.json"))
                ?? dir.lastPathComponent
            return ProjectInfo(path: dir.path, name: name, kind: .node)
        }
        if has("go.mod") { return ProjectInfo(path: dir.path, name: dir.lastPathComponent, kind: .go) }
        if has("pyproject.toml") || has("requirements.txt") {
            return ProjectInfo(path: dir.path, name: dir.lastPathComponent, kind: .python)
        }
        if has("Gemfile") { return ProjectInfo(path: dir.path, name: dir.lastPathComponent, kind: .ruby) }
        if has("Cargo.toml") { return ProjectInfo(path: dir.path, name: dir.lastPathComponent, kind: .rust) }
        if has(".git") { return ProjectInfo(path: dir.path, name: dir.lastPathComponent, kind: .unknown) }
        return nil
    }

    private static func packageName(at url: URL) -> String? {
        guard let data = try? Data(contentsOf: url),
              let obj = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
              let name = obj["name"] as? String, !name.isEmpty else { return nil }
        return name
    }
}
