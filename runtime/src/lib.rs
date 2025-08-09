#![cfg_attr(not(feature = "std"), no_std)]

use sp_api::impl_runtime_apis;
use sp_runtime::OpaqueMetadata;

// ---------- Runtime Version (sp-version v39, polkadot-stable2503-8) ----------
pub const VERSION: sp_version::RuntimeVersion = sp_version::RuntimeVersion {
    spec_name: sp_version::create_runtime_str!("solochain"),
    impl_name: sp_version::create_runtime_str!("solochain"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS, // required on this SDK
    transaction_version: 1,
    system_version: 1,          // required on sp-version v39
};

// ---------- Construct Runtime ----------
// NOTE: Replace the pallet lines with your actual pallets. This example assumes a pallet named `messagehash`.
frame_support::construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        // Example pallet — change `messagehash` to your crate name if different:
        MessageHash: messagehash::{Pallet, Call, Storage, Event<T>},
    }
);

// ---------- Pallet Config Implementations ----------
impl messagehash::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxItems = frame_support::traits::ConstU32<1024>;
    type MaxLen   = frame_support::traits::ConstU32<1024>;
}

// ---------- Executive ----------
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

// ---------- Runtime API Implementations (must be AFTER construct_runtime!) ----------
sp_api::impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> sp_version::RuntimeVersion {
            VERSION
        }
        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }
        fn initialize_block(header: &<Block as sp_runtime::traits::Block>::Header) {
            Executive::initialize_block(header);
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as sp_runtime::traits::Block>::Extrinsic)
            -> sp_runtime::ApplyExtrinsicResult
        {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as sp_runtime::traits::Block>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData)
            -> Vec<<Block as sp_runtime::traits::Block>::Extrinsic>
        {
            data.create_extrinsics()
        }

        fn check_inherents(block: Block, data: sp_inherents::InherentData)
            -> sp_inherents::CheckInherentsResult
        {
            data.check_extrinsics(&block)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }
    }

    // Add more runtime APIs here if your chain uses them (Aura/Babe, Grandpa, Offchain, TxPool, etc.)
}
