import XCTest
@testable import PortholeCore

final class TailscaleParserTests: XCTestCase {
    func testParsesServeStatus() {
        let output = """
        https://macbook.tail1234.ts.net (tailnet only)
        |-- / proxy http://127.0.0.1:3000

        https://macbook.tail1234.ts.net:8443 (Funnel on)
        |-- / proxy http://127.0.0.1:8080
        """
        XCTAssertEqual(TailscaleParser.parse(output), [
            TunnelInfo(provider: .tailscale, publicURL: "https://macbook.tail1234.ts.net", targetPort: 3000),
            TunnelInfo(provider: .tailscale, publicURL: "https://macbook.tail1234.ts.net:8443", targetPort: 8080),
        ])
    }

    func testHandlesNoServe() {
        XCTAssertEqual(TailscaleParser.parse("No serve config\n"), [])
    }
}
