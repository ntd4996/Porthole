import XCTest
@testable import PortholeCore

final class ProjectResolverTests: XCTestCase {
    private func makeTempDir() throws -> URL {
        let dir = FileManager.default.temporaryDirectory
            .appendingPathComponent("porthole-test-\(UUID().uuidString)")
        try FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        return dir
    }

    func testReadsNameFromPackageJSON() throws {
        let root = try makeTempDir()
        defer { try? FileManager.default.removeItem(at: root) }
        try #"{"name":"roomify"}"#.write(
            to: root.appendingPathComponent("package.json"), atomically: true, encoding: .utf8)
        let sub = root.appendingPathComponent("src/server")
        try FileManager.default.createDirectory(at: sub, withIntermediateDirectories: true)

        let info = ProjectResolver.resolve(cwd: sub.path)
        XCTAssertEqual(info?.name, "roomify")
        XCTAssertEqual(info?.kind, .node)
        XCTAssertEqual(info?.path, root.path)
    }

    func testGitRootFallbackUsesBasename() throws {
        let root = try makeTempDir()
        defer { try? FileManager.default.removeItem(at: root) }
        try FileManager.default.createDirectory(
            at: root.appendingPathComponent(".git"), withIntermediateDirectories: true)

        let info = ProjectResolver.resolve(cwd: root.path)
        XCTAssertEqual(info?.name, root.lastPathComponent)
        XCTAssertEqual(info?.kind, .unknown)
    }

    func testReturnsNilWhenNoMarker() throws {
        let root = try makeTempDir()
        defer { try? FileManager.default.removeItem(at: root) }
        XCTAssertNil(ProjectResolver.resolve(cwd: root.path))
    }
}
