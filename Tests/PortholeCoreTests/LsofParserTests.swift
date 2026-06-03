import XCTest
@testable import PortholeCore

final class LsofParserTests: XCTestCase {
    func testParsesMultipleProcessesAndDedupsIPv4IPv6() {
        let output = """
        p1234
        cnode
        n127.0.0.1:3000
        n[::1]:3000
        p5678
        credis-ser
        n*:6379
        """
        let result = LsofParser.parseListens(output)
        XCTAssertEqual(result, [
            RawListen(pid: 1234, command: "node", port: 3000),
            RawListen(pid: 5678, command: "redis-ser", port: 6379),
        ])
    }

    func testIgnoresMalformedLinesAndEmptyInput() {
        XCTAssertEqual(LsofParser.parseListens(""), [])
        XCTAssertEqual(LsofParser.parseListens("garbage\nf12\n"), [])
    }
}
