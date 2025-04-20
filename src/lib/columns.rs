//! Column definitions for each database

pub mod compression;
pub mod gas_price;
pub mod off_chain;
pub mod on_chain;
pub mod relayer;

pub use self::{
    compression::CompressionColumn,
    gas_price::GasPriceColumn,
    off_chain::OffChainColumn,
    on_chain::OnchainColumn,
    relayer::RelayerColumn,
};

/// Column definitions for each database
#[derive(Debug, Clone)]
pub enum Column {
    /// Onchain column
    Onchain(OnchainColumn),
    /// Offchain column
    Offchain(OffChainColumn),
    /// Compression column
    Compression(CompressionColumn),
    /// Gas price column
    GasPrice(GasPriceColumn),
    /// Relayer column
    Relayer(RelayerColumn),
}

impl Column {
    /// Returns the onchain column if it exists
    pub fn as_onchain(&self) -> Option<&OnchainColumn> {
        match self {
            Self::Onchain(column) => Some(column),
            _ => None,
        }
    }

    /// Returns the offchain column if it exists
    pub fn as_offchain(&self) -> Option<&OffChainColumn> {
        match self {
            Self::Offchain(column) => Some(column),
            _ => None,
        }
    }

    /// Returns the compression column if it exists
    pub fn as_compression(&self) -> Option<&CompressionColumn> {
        match self {
            Self::Compression(column) => Some(column),
            _ => None,
        }
    }

    /// Returns the gas price column if it exists
    pub fn as_gas_price(&self) -> Option<&GasPriceColumn> {
        match self {
            Self::GasPrice(column) => Some(column),
            _ => None,
        }
    }

    /// Returns the relayer column if it exists
    pub fn as_relayer(&self) -> Option<&RelayerColumn> {
        match self {
            Self::Relayer(column) => Some(column),
            _ => None,
        }
    }
}

impl From<OnchainColumn> for Column {
    fn from(column: OnchainColumn) -> Self {
        Self::Onchain(column)
    }
}

impl From<OffChainColumn> for Column {
    fn from(column: OffChainColumn) -> Self {
        Self::Offchain(column)
    }
}

impl From<CompressionColumn> for Column {
    fn from(column: CompressionColumn) -> Self {
        Self::Compression(column)
    }
}

impl From<GasPriceColumn> for Column {
    fn from(column: GasPriceColumn) -> Self {
        Self::GasPrice(column)
    }
}

impl From<RelayerColumn> for Column {
    fn from(column: RelayerColumn) -> Self {
        Self::Relayer(column)
    }
}
