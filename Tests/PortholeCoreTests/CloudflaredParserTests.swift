import XCTest
@testable import PortholeCore

final class CloudflaredParserTests: XCTestCase {
    func testTargetPortFromCommandLine() {
        XCTAssertEqual(
            CloudflaredParser.targetPort(fromCommandLine: "cloudflared tunnel --url http://localhost:3000"),
            3000)
        XCTAssertEqual(
            CloudflaredParser.targetPort(fromCommandLine: "cloudflared tunnel --url 127.0.0.1:8787 run"),
            8787)
        XCTAssertNil(CloudflaredParser.targetPort(fromCommandLine: "cloudflared tunnel run mytun"))
    }

    func testIngressFromConfigYAML() {
        let yaml = """
        tunnel: abc-123
        ingress:
          - hostname: app.example.com
            service: http://localhost:3000
          - hostname: api.example.com
            service: http://localhost:8080
          - service: http_status:404
        """
        XCTAssertEqual(CloudflaredParser.ingress(fromConfigYAML: yaml), [
            TunnelInfo(provider: .cloudflare, publicURL: "https://app.example.com", targetPort: 3000),
            TunnelInfo(provider: .cloudflare, publicURL: "https://api.example.com", targetPort: 8080),
        ])
    }
}
