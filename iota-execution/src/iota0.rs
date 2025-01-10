use std::{collections::HashSet, path::PathBuf, sync::Arc};

use iota_adapter_latest::{
    adapter::{new_move_vm, run_metered_move_bytecode_verifier},
    execution_engine::{execute_genesis_state_update, execute_transaction_to_effects},
    execution_mode,
    type_layout_resolver::TypeLayoutResolver,
};
use iota_move_natives_latest::all_natives;
use iota_protocol_config::ProtocolConfig;
use iota_types::{
    base_types::{IotaAddress, ObjectRef, TxContext},
    committee::EpochId,
    digests::TransactionDigest,
    effects::TransactionEffects,
    error::{ExecutionError, IotaError, IotaResult},
    execution::{ExecutionResult, TypeLayoutStore},
    gas::IotaGasStatus,
    inner_temporary_store::InnerTemporaryStore,
    layout_resolver::LayoutResolver,
    metrics::{BytecodeVerifierMetrics, LimitsMetrics},
    storage::BackingStore,
    transaction::{CheckedInputObjects, ProgrammableTransaction, TransactionKind},
};
use iota_verifier_latest::meter::IotaVerifierMeter;
use move_binary_format::CompiledModule;
use move_bytecode_verifier_meter::Meter;
use move_vm_config::verifier::{MeterConfig, VerifierConfig};
use move_vm_runtime_latest::move_vm::MoveVM;
// use iota0_core;

pub(crate) struct Executor;

pub(crate) fn executor(
    protocol_config: &ProtocolConfig,
    silent: bool,
    enable_profiler: Option<PathBuf>,
) -> Result<Executor, IotaError> {
    todo!()
}

impl crate::executor::Executor for Executor {
    fn execute_transaction_to_effects(
        &self,
        store: &dyn BackingStore,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
        enable_expensive_checks: bool,
        certificate_deny_set: &HashSet<TransactionDigest>,
        epoch_id: &EpochId,
        epoch_timestamp_ms: u64,
        input_objects: CheckedInputObjects,
        gas_coins: Vec<ObjectRef>,
        gas_status: IotaGasStatus,
        transaction_kind: TransactionKind,
        transaction_signer: IotaAddress,
        transaction_digest: TransactionDigest,
    ) -> (
        InnerTemporaryStore,
        IotaGasStatus,
        TransactionEffects,
        Result<(), ExecutionError>,
    ) {
        todo!()
    }

    fn dev_inspect_transaction(
        &self,
        store: &dyn BackingStore,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
        enable_expensive_checks: bool,
        certificate_deny_set: &HashSet<TransactionDigest>,
        epoch_id: &EpochId,
        epoch_timestamp_ms: u64,
        input_objects: CheckedInputObjects,
        gas_coins: Vec<ObjectRef>,
        gas_status: IotaGasStatus,
        transaction_kind: TransactionKind,
        transaction_signer: IotaAddress,
        transaction_digest: TransactionDigest,
        skip_all_checks: bool,
    ) -> (
        InnerTemporaryStore,
        IotaGasStatus,
        TransactionEffects,
        Result<Vec<ExecutionResult>, ExecutionError>,
    ) {
        todo!()
    }

    fn update_genesis_state(
        &self,
        store: &dyn BackingStore,
        protocol_config: &ProtocolConfig,
        metrics: Arc<LimitsMetrics>,
        tx_context: &mut TxContext,
        input_objects: CheckedInputObjects,
        pt: ProgrammableTransaction,
    ) -> Result<InnerTemporaryStore, ExecutionError> {
        todo!()
    }

    fn type_layout_resolver<'r, 'vm: 'r, 'store: 'r>(
        &'vm self,
        store: Box<dyn TypeLayoutStore + 'store>,
    ) -> Box<dyn LayoutResolver + 'r> {
        todo!()
    }
}
