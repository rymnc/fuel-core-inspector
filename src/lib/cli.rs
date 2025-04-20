//! CLI interface definition

use crate::ParseColumnForDatabase;
use clap::Parser;

/// CLI args
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "This is a fuel-core swiss army knife to inspect and mutate database k-v pairs"
)]
#[command(rename_all = "snake_case")]
pub struct FuelCoreInspectorCliArgs {
    /// Command to execute
    #[clap(subcommand)]
    command: Command,
}

/// Command configuration
#[derive(clap::Args, Debug, Clone)]
pub struct CmdConfig {
    /// Database name
    #[arg(long)]
    database: crate::databases::Database,

    /// Database config
    #[clap(flatten)]
    database_config: crate::databases::DatabaseConfig,

    /// Column name
    #[arg(long, short)]
    column: String,

    /// Key to inspect
    #[arg(long, short)]
    key: String,

    /// Value to write
    #[arg(long, short)]
    value: Option<String>,
}

#[derive(clap::Subcommand, Debug, Clone)]
#[clap(rename_all = "snake_case")]
enum Command {
    /// Inspect database k-v pairs
    Inspect(CmdConfig),
    /// Mutate database k-v pairs
    Mutate(CmdConfig),
}

impl Command {
    fn into_cmd_config(self) -> CmdConfig {
        match self {
            Command::Inspect(config) => config,
            Command::Mutate(config) => config,
        }
    }
}

/// Validated FuelCoreInspectorCliArgs
#[derive(Debug, Clone)]
pub struct ValidatedFuelCoreInspectorCliArgs {
    /// Database name
    database: crate::databases::Database,
    /// Database config
    database_config: crate::databases::DatabaseConfig,
    /// column
    column: crate::columns::Column,
    /// key
    key: String,
    /// value
    value: String,
    /// command
    cmd: CommandWithoutConfig,
}

/// command without config
#[derive(Debug, Clone, Copy)]
pub enum CommandWithoutConfig {
    /// Inspect database k-v pairs
    Inspect,
    /// Mutate database k-v pairs
    Mutate,
}

impl ValidatedFuelCoreInspectorCliArgs {
    /// column
    pub fn column(&self) -> &crate::columns::Column {
        &self.column
    }

    /// key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// database
    pub fn database(&self) -> &crate::databases::Database {
        &self.database
    }

    /// database config
    pub fn database_config(&self) -> &crate::databases::DatabaseConfig {
        &self.database_config
    }

    /// command
    pub fn cmd(&self) -> &CommandWithoutConfig {
        &self.cmd
    }
}

impl FuelCoreInspectorCliArgs {
    /// validate and parse the column into an enum variant
    pub fn validate(self) -> anyhow::Result<ValidatedFuelCoreInspectorCliArgs> {
        let cmd = match self.command {
            Command::Inspect(_) => CommandWithoutConfig::Inspect,
            Command::Mutate(_) => CommandWithoutConfig::Mutate,
        };
        let CmdConfig {
            database,
            column,
            key,
            database_config,
            value,
        } = self.command.into_cmd_config();

        if matches!(cmd, CommandWithoutConfig::Mutate) && value.is_none() {
            return Err(anyhow::anyhow!("Value is required for mutate command"));
        }

        let column = database.parse_column_for_database(column.as_str())?;

        if !std::path::Path::new(&database_config.path).exists() {
            return Err(anyhow::anyhow!(
                "Database path `{}` does not exist",
                database_config.path
            ));
        }

        Ok(ValidatedFuelCoreInspectorCliArgs {
            database,
            column,
            key,
            database_config,
            cmd,
            value: value.unwrap_or_default(),
        })
    }
}
