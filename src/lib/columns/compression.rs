//! Column definition for compression database

use fuel_core_compression_service::storage::column;
use fuel_core_storage::merkle::column::MerkleizedColumn;

/// Column definition for compression database
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
pub enum CompressionColumn {
    /// CompressedBlocks, see [`CompressedBlocks`](crate::storage::compressed_blocks::CompressedBlocks)
    CompressedBlocks = 0,
    /// RegistryKey to Address index, see [`Address`](crate::storage::address::Address)
    Address = 1,
    /// RegistryKey to AssetId index, see [`AssetId`](crate::storage::asset_id::AssetId)
    AssetId = 2,
    /// RegistryKey to ContractId index, see [`ContractId`](crate::storage::contract_id::ContractId)
    ContractId = 3,
    /// RegistryKey to ScriptCode index, see [`ScriptCode`](crate::storage::script_code::ScriptCode)
    ScriptCode = 4,
    /// RegistryKey to PredicateCode index, see [`PredicateCode`](crate::storage::predicate_code::PredicateCode)
    PredicateCode = 5,
    /// RegistryKey to ReverseKey index, see [`RegistryIndex`](crate::storage::registry_index::RegistryIndex)
    RegistryIndex = 6,
    /// Keeps track of keys to remove, see [`EvictorCache`](crate::storage::evictor_cache::EvictorCache)
    EvictorCache = 7,
    /// Keeps track of timestamps, will be removed eventually, see [`Timestamps`](crate::storage::timestamps::Timestamps)
    Timestamps = 8,
}

impl From<CompressionColumn> for MerkleizedColumn<column::CompressionColumn> {
    fn from(value: CompressionColumn) -> Self {
        match value {
            CompressionColumn::CompressedBlocks => {
                Self::TableColumn(column::CompressionColumn::CompressedBlocks)
            }
            CompressionColumn::Address => {
                Self::TableColumn(column::CompressionColumn::Address)
            }
            CompressionColumn::AssetId => {
                Self::TableColumn(column::CompressionColumn::AssetId)
            }
            CompressionColumn::ContractId => {
                Self::TableColumn(column::CompressionColumn::ContractId)
            }
            CompressionColumn::ScriptCode => {
                Self::TableColumn(column::CompressionColumn::ScriptCode)
            }
            CompressionColumn::PredicateCode => {
                Self::TableColumn(column::CompressionColumn::PredicateCode)
            }
            CompressionColumn::RegistryIndex => {
                Self::TableColumn(column::CompressionColumn::RegistryIndex)
            }
            CompressionColumn::EvictorCache => {
                Self::TableColumn(column::CompressionColumn::EvictorCache)
            }
            CompressionColumn::Timestamps => {
                Self::TableColumn(column::CompressionColumn::Timestamps)
            }
        }
    }
}
