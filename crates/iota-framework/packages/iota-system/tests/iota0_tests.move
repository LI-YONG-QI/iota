// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module iota_system::iota0_tests {
    use iota_system::iota0;

    #[test]
    fun test_verify_iota0_fake_receipt() {
        let inner = x"";
        let journal = x"";

        let result = iota0::verify_iota0_receipt(&inner, &journal);
        assert!(result == true);
    }
}
