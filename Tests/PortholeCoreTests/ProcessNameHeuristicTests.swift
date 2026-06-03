import XCTest
@testable import PortholeCore

final class ProcessNameHeuristicTests: XCTestCase {
    func testDetectsKnownDevTools() {
        XCTAssertEqual(
            ProcessNameHeuristic.displayName(
                command: "node",
                fullCommand: "node /Users/x/app/node_modules/.bin/vite"),
            "vite")
        XCTAssertEqual(
            ProcessNameHeuristic.displayName(
                command: "node",
                fullCommand: "next-server (v14.2.0)"),
            "next")
        XCTAssertEqual(
            ProcessNameHeuristic.displayName(
                command: "python3.12",
                fullCommand: "/usr/bin/python3.12 -m uvicorn main:app"),
            "uvicorn")
    }

    func testFallsBackToCommand() {
        XCTAssertEqual(
            ProcessNameHeuristic.displayName(command: "redis-server", fullCommand: "redis-server *:6379"),
            "redis-server")
    }
}
