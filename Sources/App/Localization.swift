import Foundation

/// Localizes a key against this module's bundle (SPM resources live in
/// `Bundle.module`, not `.main`, so SwiftUI's automatic lookup misses them).
/// The English source string is the key; translations live in `*.lproj`.
func loc(_ key: String.LocalizationValue) -> String {
    String(localized: key, bundle: .module)
}
