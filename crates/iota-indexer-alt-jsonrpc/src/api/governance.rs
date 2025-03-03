// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use diesel::{ExpressionMethods, QueryDsl};
use iota_indexer_alt_schema::schema::kv_epoch_starts;
use iota_pg_db::Db;
use iota_types::iota_serde::BigInt;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

use super::Connection;

#[rpc(server, namespace = "iotax")]
trait Governance {
    /// Return the reference gas price for the network as of the latest epoch.
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;
}

pub(crate) struct GovernanceImpl(pub Db);

#[async_trait::async_trait]
impl GovernanceServer for GovernanceImpl {
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        use kv_epoch_starts::dsl as e;

        let mut conn = Connection::get(&self.0).await?;
        let rgp: i64 = conn
            .first(
                e::kv_epoch_starts
                    .select(e::reference_gas_price)
                    .order(e::epoch.desc()),
            )
            .await?;

        Ok((rgp as u64).into())
    }
}
