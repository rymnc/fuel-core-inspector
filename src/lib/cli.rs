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
    key: std::sync::Arc<[u8]>,
    /// value
    value: std::sync::Arc<[u8]>,
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
    pub fn key(&self) -> &[u8] {
        &self.key
    }

    /// value
    pub fn value(&self) -> &[u8] {
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

fn hex_string_to_bytes<S>(hex_string: S) -> anyhow::Result<std::sync::Arc<[u8]>>
where
    S: AsRef<str>,
{
    let hex_string = hex_string
        .as_ref()
        .strip_prefix("0x")
        .unwrap_or(hex_string.as_ref());

    if hex_string.len() % 2 != 0 {
        anyhow::bail!("Hex string must have an even length");
    }

    let mut bytes = Vec::with_capacity(hex_string.len() / 2);

    let mut chars = hex_string.chars();
    while let (Some(a), Some(b)) = (chars.next(), chars.next()) {
        let byte = u8::from_str_radix(&format!("{}{}", a, b), 16)?;
        bytes.push(byte);
    }
    Ok(std::sync::Arc::from(bytes))
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

        let key = hex_string_to_bytes(&key)?;
        let value = value.map(hex_string_to_bytes).transpose()?;

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
