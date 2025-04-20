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

// defined as static var so it can be reused when we have repl mode
static DB: std::sync::OnceLock<anyhow::Result<CombinedDatabase>> =
    std::sync::OnceLock::new();

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
}

impl DatabaseHandle {
    /// Create a new database handle
    pub fn new(variant: Database, config: DatabaseConfig) -> Self {
        Self { variant, config }
    }

    /// Get the database variant
    pub fn variant(&self) -> Database {
        self.variant
    }

    /// Get the database configuration
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    fn db(&self) -> anyhow::Result<&'static CombinedDatabase> {
        let res = DB.get_or_init(|| {
            let path = PathBuf::from_str(&self.config.path)?;
            // TODO: make configurable
            let state_rewind_policy = StateRewindPolicy::NoRewind;
            let db_config = fuel_core::state::rocks_db::DatabaseConfig {
                cache_capacity: None,
                max_fds: -1,
                columns_policy: ColumnsPolicy::Lazy,
            };

            // only open the variant's database, rest are in memory
            // TODO: in repl mode, maybe we want to open all databases, using CombinedDatabase::open
            match self.variant() {
                Database::OnChain => {
                    let db = fuel_core::database::Database::open_rocksdb(
                        &path,
                        state_rewind_policy,
                        db_config,
                    )?;

                    let combined_database = CombinedDatabase::new(
                        db,
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    );
                    Ok(combined_database)
                }
                Database::OffChain => {
                    let db = fuel_core::database::Database::open_rocksdb(
                        &path,
                        state_rewind_policy,
                        db_config,
                    )?;

                    let combined_database = CombinedDatabase::new(
                        Default::default(),
                        db,
                        Default::default(),
                        Default::default(),
                        Default::default(),
                    );
                    Ok(combined_database)
                }
                Database::Relayer => {
                    let db = fuel_core::database::Database::open_rocksdb(
                        &path,
                        state_rewind_policy,
                        db_config,
                    )?;
                    let combined_database = CombinedDatabase::new(
                        Default::default(),
                        Default::default(),
                        db,
                        Default::default(),
                        Default::default(),
                    );
                    Ok(combined_database)
                }
                Database::GasPrice => {
                    let db = fuel_core::database::Database::open_rocksdb(
                        &path,
                        state_rewind_policy,
                        db_config,
                    )?;
                    let combined_database = CombinedDatabase::new(
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        db,
                        Default::default(),
                    );
                    Ok(combined_database)
                }
                Database::Compression => {
                    let db = fuel_core::database::Database::open_rocksdb(
                        &path,
                        state_rewind_policy,
                        db_config,
                    )?;
                    let combined_database = CombinedDatabase::new(
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        Default::default(),
                        db,
                    );
                    Ok(combined_database)
                }
            }
        });

        match res {
            Ok(db) => Ok(db),
            Err(err) => anyhow::bail!(err.to_string()),
        }
    }

    /// Perform a read operation on the database
    pub fn perform_read(
        &self,
        column: &Column,
        key: &[u8],
    ) -> anyhow::Result<Option<Value>> {
        // Implementation of read operation
        let db = self.db()?;

        let maybe_value = match self.variant() {
            Database::OnChain => db.on_chain().get(
                key,
                column
                    .as_onchain()
                    .ok_or_else(|| anyhow::anyhow!("invalid variant"))?
                    .clone()
                    .into(),
            )?,
            Database::OffChain => db.off_chain().get(
                key,
                column
                    .as_offchain()
                    .ok_or_else(|| anyhow::anyhow!("invalid variant"))?
                    .clone()
                    .into(),
            )?,
            Database::Compression => db.compression().get(
                key,
                column
                    .as_compression()
                    .ok_or_else(|| anyhow::anyhow!("invalid variant"))?
                    .clone()
                    .into(),
            )?,
            Database::GasPrice => db.gas_price().get(
                key,
                column
                    .as_gas_price()
                    .ok_or_else(|| anyhow::anyhow!("invalid variant"))?
                    .clone()
                    .into(),
            )?,
            Database::Relayer => db.relayer().get(
                key,
                column
                    .as_relayer()
                    .ok_or_else(|| anyhow::anyhow!("invalid variant"))?
                    .clone()
                    .into(),
            )?,
        };

        Ok(maybe_value)
    }

    /// Perform a write operation on the database
    pub fn perform_write(
        &self,
        column: &Column,
        key: &[u8],
        value: &[u8],
    ) -> anyhow::Result<()> {
        // Implementation of write operation
        let mut db = self.db()?.clone();

        match self.variant() {
            Database::OnChain => {
                let mut tx = db.on_chain_mut().write_transaction();
                tx.write(
                    key,
                    column
                        .as_onchain()
                        .ok_or_else(|| anyhow::anyhow!("invalid variant"))?
                        .clone()
                        .into(),
                    value,
                )?;
                tx.commit()?;
            }
            Database::OffChain => {
                let mut tx = db.off_chain_mut().write_transaction();
                tx.write(
                    key,
                    column
                        .as_offchain()
                        .ok_or_else(|| anyhow::anyhow!("invalid variant"))?
                        .clone()
                        .into(),
                    value,
                )?;
                tx.commit()?;
            }
            Database::Compression => {
                panic!("Compression database is not supported for write operations");
            }
            Database::GasPrice => {
                let mut tx = db.gas_price_mut().write_transaction();
                tx.write(
                    key,
                    column
                        .as_gas_price()
                        .ok_or_else(|| anyhow::anyhow!("invalid variant"))?
                        .clone()
                        .into(),
                    value,
                )?;
                tx.commit()?;
            }
            Database::Relayer => {
                let mut tx = db.relayer_mut().write_transaction();
                tx.write(
                    key,
                    column
                        .as_relayer()
                        .ok_or_else(|| anyhow::anyhow!("invalid variant"))?
                        .clone()
                        .into(),
                    value,
                )?;
                tx.commit()?;
            }
        }

        Ok(())
    }
}
