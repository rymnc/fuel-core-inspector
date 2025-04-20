//! Column definition for on-chain database

use fuel_core_storage::column;

/// Column definition for on-chain database
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    enum_iterator::Sequence,
    serde::Serialize,
    serde::Deserialize,
    clap::ValueEnum,
    strum::EnumString,
    strum::VariantNames,
)]
#[strum(serialize_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum OnchainColumn {
    /// The column id of metadata about the blockchain
    Metadata = 0,
    /// See [`ContractsRawCode`](crate::tables::ContractsRawCode)
    ContractsRawCode = 1,
    /// See [`ContractsState`](crate::tables::ContractsState)
    ContractsState = 2,
    /// See [`ContractsLatestUtxo`](crate::tables::ContractsLatestUtxo)
    ContractsLatestUtxo = 3,
    /// See [`ContractsAssets`](crate::tables::ContractsAssets)
    ContractsAssets = 4,
    /// See [`Coins`](crate::tables::Coins)
    Coins = 5,
    /// See [`Transactions`](crate::tables::Transactions)
    Transactions = 6,
    /// See [`FuelBlocks`](crate::tables::FuelBlocks)
    FuelBlocks = 7,
    /// See [`FuelBlockMerkleData`](crate::tables::merkle::FuelBlockMerkleData)
    FuelBlockMerkleData = 8,
    /// See [`FuelBlockMerkleMetadata`](crate::tables::merkle::FuelBlockMerkleMetadata)
    FuelBlockMerkleMetadata = 9,
    /// See [`ContractsAssetsMerkleData`](crate::tables::merkle::ContractsAssetsMerkleData)
    ContractsAssetsMerkleData = 10,
    /// See [`ContractsAssetsMerkleMetadata`](crate::tables::merkle::ContractsAssetsMerkleMetadata)
    ContractsAssetsMerkleMetadata = 11,
    /// See [`ContractsStateMerkleData`](crate::tables::merkle::ContractsStateMerkleData)
    ContractsStateMerkleData = 12,
    /// See [`ContractsStateMerkleMetadata`](crate::tables::merkle::ContractsStateMerkleMetadata)
    ContractsStateMerkleMetadata = 13,
    /// See [`Messages`](crate::tables::Messages)
    Messages = 14,
    /// See [`ProcessedTransactions`](crate::tables::ProcessedTransactions)
    ProcessedTransactions = 15,
    /// See [`SealedBlockConsensus`](crate::tables::SealedBlockConsensus)
    FuelBlockConsensus = 16,
    /// See [`ConsensusParametersVersions`](crate::tables::ConsensusParametersVersions)
    ConsensusParametersVersions = 17,
    /// See [`StateTransitionBytecodeVersions`](crate::tables::StateTransitionBytecodeVersions)
    StateTransitionBytecodeVersions = 18,
    /// See [`UploadedBytecodes`](crate::tables::UploadedBytecodes)
    UploadedBytecodes = 19,
    /// See [`Blobs`](fuel_vm_private::storage::BlobData)
    Blobs = 20,

    // TODO: Remove this column and use `Metadata` column instead.
    /// Table for genesis state import progress tracking.
    GenesisMetadata = 21,
}

impl From<OnchainColumn> for column::Column {
    fn from(value: OnchainColumn) -> Self {
        match value {
            OnchainColumn::Metadata => Self::Metadata,
            OnchainColumn::ContractsRawCode => Self::ContractsRawCode,
            OnchainColumn::ContractsState => Self::ContractsState,
            OnchainColumn::ContractsLatestUtxo => Self::ContractsLatestUtxo,
            OnchainColumn::ContractsAssets => Self::ContractsAssets,
            OnchainColumn::Coins => Self::Coins,
            OnchainColumn::Transactions => Self::Transactions,
            OnchainColumn::FuelBlocks => Self::FuelBlocks,
            OnchainColumn::FuelBlockMerkleData => Self::FuelBlockMerkleData,
            OnchainColumn::FuelBlockMerkleMetadata => Self::FuelBlockMerkleMetadata,
            OnchainColumn::ContractsAssetsMerkleData => Self::ContractsAssetsMerkleData,
            OnchainColumn::ContractsAssetsMerkleMetadata => {
                Self::ContractsAssetsMerkleMetadata
            }
            OnchainColumn::ContractsStateMerkleData => Self::ContractsStateMerkleData,
            OnchainColumn::ContractsStateMerkleMetadata => {
                Self::ContractsStateMerkleMetadata
            }
            OnchainColumn::Messages => Self::Messages,
            OnchainColumn::ProcessedTransactions => Self::ProcessedTransactions,
            OnchainColumn::FuelBlockConsensus => Self::FuelBlockConsensus,
            OnchainColumn::ConsensusParametersVersions => {
                Self::ConsensusParametersVersions
            }
            OnchainColumn::StateTransitionBytecodeVersions => {
                Self::StateTransitionBytecodeVersions
            }
            OnchainColumn::UploadedBytecodes => Self::UploadedBytecodes,
            OnchainColumn::Blobs => Self::Blobs,
            OnchainColumn::GenesisMetadata => Self::GenesisMetadata,
        }
    }
}
