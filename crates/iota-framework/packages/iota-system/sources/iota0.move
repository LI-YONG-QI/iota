// Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// IOTA0 operations.
module iota_system::iota0 {
    public native fun verify_iota0_receipt(inner: &vector<u8>, journal: &vector<u8>): bool;
}
