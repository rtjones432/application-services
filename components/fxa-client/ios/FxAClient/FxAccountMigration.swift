/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

enum MigrationState {
    case none
    case copySessionToken
    case reuseSessionToken

    internal static func fromNumber(_ number: UInt8) -> MigrationState {
        switch number {
        case 0: return .none
        case 1: return .copySessionToken
        case 2: return .reuseSessionToken
        default: fatalError("Unreachable")
        }
    }
}

public enum MigrationResult {
    // Sign-in failed due to an intermittent problem (such as a network failure). A retry attempt will
    // be performed automatically during account manager initialization.
    // The app should try to do the same regularly (e.g. before syncing).
    case willRetry
    // Sign-in succeeded with no issues.
    // Applications may treat this account as "authenticated" after seeing this result.
    case success
    // Sign-in failed due to non-recoverable issues.
    case failure
}
