//! Swiss army knife for fuel-core database operations

#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::cast_possible_truncation)]
#![deny(unused_crate_dependencies)]
#![deny(missing_docs)]
#![deny(warnings)]

pub mod cli;
pub mod columns;
pub mod databases;
pub mod printer;

use cli::ValidatedFuelCoreInspectorCliArgs;
use databases::DatabaseHandle;
use std::str::FromStr;
use strum::VariantNames;

pub(crate) trait ParseColumnForDatabase {
    fn parse_column_for_database(&self, c: &str) -> anyhow::Result<columns::Column>;
}

impl ParseColumnForDatabase for databases::Database {
    fn parse_column_for_database(&self, c: &str) -> anyhow::Result<columns::Column> {
        fn parse_column<T: FromStr + VariantNames + Into<columns::Column>>(
            c: &str,
            db_name: &str,
        ) -> anyhow::Result<columns::Column>
        where
            T::Err: std::fmt::Debug,
        {
            let parsed = T::from_str(c).map_err(|_| {
                anyhow::anyhow!(
                    "Invalid column: \"{}\" for {} database. Expected one of {:?}",
                    c,
                    db_name,
                    T::VARIANTS
                )
            })?;

            Ok(parsed.into())
        }

        match self {
            Self::OnChain => parse_column::<crate::columns::OnchainColumn>(c, "on-chain"),
            Self::OffChain => {
                parse_column::<crate::columns::OffChainColumn>(c, "off-chain")
            }
            Self::Compression => {
                parse_column::<crate::columns::CompressionColumn>(c, "compression")
            }
            Self::GasPrice => {
                parse_column::<crate::columns::GasPriceColumn>(c, "gas-price")
            }
            Self::Relayer => parse_column::<crate::columns::RelayerColumn>(c, "relayer"),
        }
    }
}

impl TryFrom<&ValidatedFuelCoreInspectorCliArgs> for DatabaseHandle {
    type Error = anyhow::Error;

    fn try_from(value: &ValidatedFuelCoreInspectorCliArgs) -> Result<Self, Self::Error> {
        Self::try_new(*value.database(), value.database_config().clone())
    }
}
