//! Column definition for relayer database

use fuel_core::database::database_description::relayer::DummyColumn;

/// Column definition for relayer database
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
pub enum RelayerColumn {
    /// The column id of metadata about the relayer storage.
    Metadata = 0,
    /// The column of the table that stores history of the relayer.
    History = 1,
}

impl From<RelayerColumn> for fuel_core_relayer::storage::Column {
    fn from(value: RelayerColumn) -> Self {
        match value {
            RelayerColumn::Metadata => Self::Metadata,
            RelayerColumn::History => Self::History,
        }
    }
}

impl From<RelayerColumn> for DummyColumn {
    fn from(value: RelayerColumn) -> Self {
        match value {
            RelayerColumn::Metadata => Self::Metadata,
            _ => panic!("idk"),
        }
    }
}
