// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_protocol_config::ProtocolConfig;
use iota_stardust_sdk::types::block::output::{FoundryOutput, OutputId};

use crate::{
    base_types::{IotaAddress, ObjectID, SequenceNumber, TxContext},
    id::UID,
    object::Object,
    stardust::coin_type::CoinType,
};

pub fn create_foundry_amount_coin(
    output_id: &OutputId,
    foundry: &FoundryOutput,
    owner: IotaAddress,
    tx_context: &TxContext,
    version: SequenceNumber,
    protocol_config: &ProtocolConfig,
    coin_type: &CoinType,
) -> anyhow::Result<Object> {
    crate::stardust::output::create_coin(
        UID::new(ObjectID::new(output_id.hash())),
        owner,
        foundry.amount(),
        tx_context,
        version,
        protocol_config,
        coin_type,
    )
}
