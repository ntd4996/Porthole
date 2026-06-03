import XCTest
@testable import PortholeCore

final class LocaltunnelParserTests: XCTestCase {
    func testParsesPortAndSubdomain() {
        XCTAssertEqual(
            LocaltunnelParser.parse(fromCommandLine: "node lt --port 3000 --subdomain foo"),
            TunnelInfo(provider: .localtunnel, publicURL: "https://foo.loca.lt", targetPort: 3000))
    }

    func testParsesPortOnly() {
        XCTAssertEqual(
            LocaltunnelParser.parse(fromCommandLine: "lt --port 8080"),
            TunnelInfo(provider: .localtunnel, publicURL: nil, targetPort: 8080))
    }

    func testReturnsNilWithoutPort() {
        XCTAssertNil(LocaltunnelParser.parse(fromCommandLine: "lt --help"))
    }
}
