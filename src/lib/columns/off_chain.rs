//! Column definition for off-chain/indexation database

/// Column definition for off-chain database
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
pub enum OffChainColumn {
    /// The column id of metadata about the blockchain
    Metadata = 0,
    /// Metadata for genesis progress
    GenesisMetadata = 1,
    /// The column of the table that stores `true` if `owner` owns `Coin` with `coin_id`
    OwnedCoins = 2,
    /// Transaction id to current status
    TransactionStatus = 3,
    /// The column of the table of all `owner`'s transactions
    TransactionsByOwnerBlockIdx = 4,
    /// The column of the table that stores `true` if `owner` owns `Message` with `message_id`
    OwnedMessageIds = 5,
    /// The column of the table that stores statistic about the blockchain.
    Statistic = 6,
    /// See [`blocks::FuelBlockIdsToHeights`]
    FuelBlockIdsToHeights = 7,
    /// See [`ContractsInfo`](contracts::ContractsInfo)
    ContractsInfo = 8,
    /// See [`OldFuelBlocks`](old::OldFuelBlocks)
    OldFuelBlocks = 9,
    /// See [`OldFuelBlockConsensus`](old::OldFuelBlockConsensus)
    OldFuelBlockConsensus = 10,
    /// See [`OldTransactions`](old::OldTransactions)
    OldTransactions = 11,
    /// Relayed Tx ID to Layer 1 Relayed Transaction status
    RelayedTransactionStatus = 12,
    /// Messages that have been spent.
    /// Existence of a key in this column means that the message has been spent.
    /// See [`SpentMessages`](messages::SpentMessages)
    SpentMessages = 13,
    /// Coin balances per account and asset.
    CoinBalances = 23,
    /// Message balances per account.
    MessageBalances = 24,
    /// See [`AssetsInfo`](assets::AssetsInfo)
    AssetsInfo = 25,
    /// Index of the coins that are available to spend.
    CoinsToSpend = 26,
}

impl From<OffChainColumn> for fuel_core::fuel_core_graphql_api::storage::Column {
    fn from(value: OffChainColumn) -> Self {
        match value {
            OffChainColumn::Metadata => Self::Metadata,
            OffChainColumn::GenesisMetadata => Self::GenesisMetadata,
            OffChainColumn::OwnedCoins => Self::OwnedCoins,
            OffChainColumn::TransactionStatus => Self::TransactionStatus,
            OffChainColumn::TransactionsByOwnerBlockIdx => {
                Self::TransactionsByOwnerBlockIdx
            }
            OffChainColumn::OwnedMessageIds => Self::OwnedMessageIds,
            OffChainColumn::Statistic => Self::Statistic,
            OffChainColumn::FuelBlockIdsToHeights => Self::FuelBlockIdsToHeights,
            OffChainColumn::ContractsInfo => Self::ContractsInfo,
            OffChainColumn::OldFuelBlocks => Self::OldFuelBlocks,
            OffChainColumn::OldFuelBlockConsensus => Self::OldFuelBlockConsensus,
            OffChainColumn::OldTransactions => Self::OldTransactions,
            OffChainColumn::RelayedTransactionStatus => Self::RelayedTransactionStatus,
            OffChainColumn::SpentMessages => Self::SpentMessages,
            OffChainColumn::CoinBalances => Self::CoinBalances,
            OffChainColumn::MessageBalances => Self::MessageBalances,
            OffChainColumn::AssetsInfo => Self::AssetsInfo,
            OffChainColumn::CoinsToSpend => Self::CoinsToSpend,
        }
    }
}
