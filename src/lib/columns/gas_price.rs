//! Column definition for gas price database

/// Column definition for gas price database
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
pub enum GasPriceColumn {
    /// Metadata column
    Metadata = 0,
    /// State column
    State = 1,
    /// Unrecorded blocks column
    UnrecordedBlocks = 2,
    /// Latest recorded height column
    LatestRecordedHeight = 3,
}

impl From<GasPriceColumn> for fuel_core_gas_price_service::common::fuel_core_storage_adapter::storage::GasPriceColumn {
    fn from(value: GasPriceColumn) -> Self {
        match value {
            GasPriceColumn::Metadata => Self::Metadata,
            GasPriceColumn::State => Self::State,
            GasPriceColumn::UnrecordedBlocks => Self::UnrecordedBlocks,
            GasPriceColumn::LatestRecordedHeight => Self::LatestRecordedHeight,
        }
    }
}
