// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use axum::{Json, extract::State};

use super::{ApiEndpoint, RouteHandler};
use crate::{Result, RpcService, types::NodeInfo};

/// Get basic information about the state of a Node
pub struct GetNodeInfo;

impl ApiEndpoint<RpcService> for GetNodeInfo {
    fn method(&self) -> axum::http::Method {
        axum::http::Method::GET
    }

    fn path(&self) -> &'static str {
        "/"
    }

    fn handler(&self) -> RouteHandler<RpcService> {
        RouteHandler::new(self.method(), get_node_info)
    }
}

async fn get_node_info(State(state): State<RpcService>) -> Result<Json<NodeInfo>> {
    state.get_node_info().map(Json)
}
