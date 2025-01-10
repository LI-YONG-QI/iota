// Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// RISC Zero operations.
module iota::risc0 {
    public native fun verify_risc0_receipt(image_id: &vector<u8>, inner: &vector<u8>, journal: &vector<u8>): bool;
}
