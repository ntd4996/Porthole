import XCTest
@testable import PortholeCore

final class NgrokParserTests: XCTestCase {
    func testParsesTunnels() {
        let json = Data(#"""
        {"tunnels":[
          {"public_url":"https://ab12.ngrok.io","config":{"addr":"http://localhost:3000"}},
          {"public_url":"tcp://1.tcp.ngrok.io:24000","config":{"addr":"localhost:5432"}}
        ]}
        """#.utf8)
        let result = NgrokParser.parse(json)
        XCTAssertEqual(result, [
            TunnelInfo(provider: .ngrok, publicURL: "https://ab12.ngrok.io", targetPort: 3000),
            TunnelInfo(provider: .ngrok, publicURL: "tcp://1.tcp.ngrok.io:24000", targetPort: 5432),
        ])
    }

    func testHandlesEmptyAndBadJSON() {
        XCTAssertEqual(NgrokParser.parse(Data(#"{"tunnels":[]}"#.utf8)), [])
        XCTAssertEqual(NgrokParser.parse(Data("not json".utf8)), [])
    }
}
