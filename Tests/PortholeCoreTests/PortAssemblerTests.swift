import XCTest
@testable import PortholeCore

final class PortAssemblerTests: XCTestCase {
    func testAssemblesAndAttachesTunnelsByPort() {
        let listens = [
            RawListen(pid: 10, command: "node", port: 3000),
            RawListen(pid: 20, command: "redis-server", port: 6379),
        ]
        let projects: [Int32: ProjectInfo] = [
            10: ProjectInfo(path: "/p/roomify", name: "roomify", kind: .node),
        ]
        let names: [Int32: String] = [10: "vite", 20: "redis-server"]
        let tunnels = [TunnelInfo(provider: .ngrok, publicURL: "https://a.ngrok.io", targetPort: 3000)]

        let result = PortAssembler.assemble(
            listens: listens, projects: projects, displayNames: names, tunnels: tunnels)

        XCTAssertEqual(result.count, 2)
        let p3000 = result.first { $0.port == 3000 }!
        XCTAssertEqual(p3000.displayName, "vite")
        XCTAssertEqual(p3000.project?.name, "roomify")
        XCTAssertEqual(p3000.tunnels, tunnels)
        let p6379 = result.first { $0.port == 6379 }!
        XCTAssertNil(p6379.project)
        XCTAssertEqual(p6379.tunnels, [])
    }

    func testSortsProjectsThenOtherLast() {
        let listens = [
            RawListen(pid: 20, command: "redis-server", port: 6379),
            RawListen(pid: 10, command: "node", port: 3000),
        ]
        let projects: [Int32: ProjectInfo] = [
            10: ProjectInfo(path: "/p/roomify", name: "roomify", kind: .node),
        ]
        let result = PortAssembler.assemble(
            listens: listens, projects: projects, displayNames: [:], tunnels: [])
        XCTAssertEqual(result.map(\.port), [3000, 6379]) // project first, Other last
    }
}
