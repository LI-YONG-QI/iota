// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module iota::risc0_tests {
    use iota::risc0;

    #[test]
    fun test_verify_risc0_fake_receipt() {
        let image_id = x"";
        let inner = x"";
        let journal = x"";

        let result = risc0::verify_risc0_receipt(&image_id, &inner, &journal);
        assert!(result == true);
    }
}
