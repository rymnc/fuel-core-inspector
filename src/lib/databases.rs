//! Databases definitions

use std::{
    path::PathBuf,
    str::FromStr,
};

use fuel_core::{
    combined_database::CombinedDatabase,
    state::{
        historical_rocksdb::StateRewindPolicy,
        rocks_db::ColumnsPolicy,
    },
};
use fuel_core_storage::{
    kv_store::{
        KeyValueInspect,
        KeyValueMutate,
        Value,
    },
    transactional::WriteTransaction,
};

use crate::columns::Column;

/// Database variants
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
)]
#[clap(rename_all = "snake_case")]
pub enum Database {
    /// On-chain database
    OnChain,
    /// Off-chain database
    OffChain,
    /// Compression database
    Compression,
    /// Gas price database
    GasPrice,
    /// Relayer database
    Relayer,
}

/// Database configuration
#[derive(Debug, Clone, clap::Args)]
pub struct DatabaseConfig {
    /// path to database
    #[arg(long)]
    pub(crate) path: String,
}

/// Database handle
pub struct DatabaseHandle {
    /// Database variant
    variant: Database,
    /// Database configuration
    config: DatabaseConfig,
    /// database
    database: CombinedDatabase,
}

impl DatabaseHandle {
    /// Create a new database handle
    pub fn try_new(variant: Database, config: DatabaseConfig) -> anyhow::Result<Self> {
        let database = Self::db(&variant, &config)?;
        Ok(Self {
            variant,
            config,
            database,
        })
    }

    /// Get the database variant
    pub fn variant(&self) -> Database {
        self.variant
    }

    /// Get the database configuration
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    fn db(
        variant: &Database,
        config: &DatabaseConfig,
    ) -> anyhow::Result<CombinedDatabase> {
        let path = PathBuf::from_str(&config.path)?;
        // TODO: make configurable
        let state_rewind_policy = StateRewindPolicy::NoRewind;
        let db_config = fuel_core::state::rocks_db::DatabaseConfig {
            cache_capacity: None,
            max_fds: -1,
            columns_policy: ColumnsPolicy::Lazy,
        };

        // only open the variant's database, rest are in memory
        // TODO: in repl mode, maybe we want to open all databases, using CombinedDatabase::open
        let res = match variant {
            Database::OnChain => {
                let db = fuel_core::database::Database::open_rocksdb(
                    &path,
                    state_rewind_policy,
                    db_config,
                )?;

                CombinedDatabase::new(
                    db,
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                )
            }
            Database::OffChain => {
                let db = fuel_core::database::Database::open_rocksdb(
                    &path,
                    state_rewind_policy,
                    db_config,
                )?;

                CombinedDatabase::new(
                    Default::default(),
                    db,
                    Default::default(),
                    Default::default(),
                    Default::default(),
                )
            }
            Database::Relayer => {
                let db = fuel_core::database::Database::open_rocksdb(
                    &path,
                    state_rewind_policy,
                    db_config,
                )?;
                CombinedDatabase::new(
                    Default::default(),
                    Default::default(),
                    db,
                    Default::default(),
                    Default::default(),
                )
            }
            Database::GasPrice => {
                let db = fuel_core::database::Database::open_rocksdb(
                    &path,
                    state_rewind_policy,
                    db_config,
                )?;
                CombinedDatabase::new(
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    db,
                    Default::default(),
                )
            }
            Database::Compression => {
                let db = fuel_core::database::Database::open_rocksdb(
                    &path,
                    state_rewind_policy,
                    db_config,
                )?;
                CombinedDatabase::new(
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    db,
                )
            }
        };

        Ok(res)
    }

    /// Perform a read operation on the database
    pub fn perform_read(
        &self,
        column: &Column,
        key: &[u8],
    ) -> anyhow::Result<Option<Value>> {
        // Implementation of read operation
        let maybe_value = match self.variant() {
            Database::OnChain => self.database.on_chain().get(
                key,
                (*column
                    .as_onchain()
                    .ok_or_else(|| anyhow::anyhow!("invalid variant"))?)
                .into(),
            )?,
            Database::OffChain => self.database.off_chain().get(
                key,
                (*column
                    .as_offchain()
                    .ok_or_else(|| anyhow::anyhow!("invalid variant"))?)
                .into(),
            )?,
            Database::Compression => self.database.compression().get(
                key,
                (*column
                    .as_compression()
                    .ok_or_else(|| anyhow::anyhow!("invalid variant"))?)
                .into(),
            )?,
            Database::GasPrice => self.database.gas_price().get(
                key,
                (*column
                    .as_gas_price()
                    .ok_or_else(|| anyhow::anyhow!("invalid variant"))?)
                .into(),
            )?,
            Database::Relayer => self.database.relayer().get(
                key,
                (*column
                    .as_relayer()
                    .ok_or_else(|| anyhow::anyhow!("invalid variant"))?)
                .into(),
            )?,
        };

        Ok(maybe_value)
    }

    /// Perform a write operation on the database
    pub fn perform_write(
        &mut self,
        column: &Column,
        key: &[u8],
        value: &[u8],
    ) -> anyhow::Result<()> {
        // Implementation of write operation
        match self.variant() {
            Database::OnChain => {
                let mut tx = self.database.on_chain_mut().write_transaction();
                tx.write(
                    key,
                    (*column
                        .as_onchain()
                        .ok_or_else(|| anyhow::anyhow!("invalid variant"))?)
                    .into(),
                    value,
                )?;
                tx.commit()?;
            }
            Database::OffChain => {
                let mut tx = self.database.off_chain_mut().write_transaction();
                tx.write(
                    key,
                    (*column
                        .as_offchain()
                        .ok_or_else(|| anyhow::anyhow!("invalid variant"))?)
                    .into(),
                    value,
                )?;
                tx.commit()?;
            }
            Database::Compression => {
                panic!("Compression database is not supported for write operations");
            }
            Database::GasPrice => {
                let mut tx = self.database.gas_price_mut().write_transaction();
                tx.write(
                    key,
                    (*column
                        .as_gas_price()
                        .ok_or_else(|| anyhow::anyhow!("invalid variant"))?)
                    .into(),
                    value,
                )?;
                tx.commit()?;
            }
            Database::Relayer => {
                let mut tx = self.database.relayer_mut().write_transaction();
                tx.write(
                    key,
                    (*column
                        .as_relayer()
                        .ok_or_else(|| anyhow::anyhow!("invalid variant"))?)
                    .into(),
                    value,
                )?;
                tx.commit()?;
            }
        }

        Ok(())
    }

    /// shutdown rocksdb
    pub fn shutdown(self) {
        self.database.shutdown();
    }
}
