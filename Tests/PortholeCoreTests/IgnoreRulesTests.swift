import XCTest
@testable import PortholeCore

final class IgnoreRulesTests: XCTestCase {
    private func port(_ p: Int, command: String, displayName: String? = nil) -> PortInfo {
        PortInfo(port: p, pid: 1, command: command, displayName: displayName ?? command)
    }

    func testIgnoresByPort() {
        let rules = IgnoreRules(ports: [7000])
        XCTAssertTrue(rules.isIgnored(port(7000, command: "ControlCenter")))
        XCTAssertFalse(rules.isIgnored(port(3000, command: "node")))
    }

    func testIgnoresByProcessCaseInsensitiveOnCommandOrDisplayName() {
        let rules = IgnoreRules(processes: ["rapportd"])
        XCTAssertTrue(rules.isIgnored(port(52972, command: "rapportd")))
        XCTAssertTrue(rules.isIgnored(port(53825, command: "RAPPORTD")))
        // match on displayName too
        let rules2 = IgnoreRules(processes: ["vite"])
        XCTAssertTrue(rules2.isIgnored(port(5173, command: "node", displayName: "vite")))
    }

    func testNotIgnoredWhenNoMatch() {
        let rules = IgnoreRules(processes: ["ControlCenter"], ports: [7000])
        XCTAssertFalse(rules.isIgnored(port(3000, command: "node", displayName: "vite")))
    }
}
