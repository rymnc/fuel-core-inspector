use clap::Parser;
use fuel_core_inspector::{
    cli::{
        CommandWithoutConfig,
        FuelCoreInspectorCliArgs,
    },
    databases::DatabaseHandle,
    printer::PrintHexExt,
};

fn hex_string_to_bytes(hex_string: &str) -> anyhow::Result<Vec<u8>> {
    let hex_string = hex_string.strip_prefix("0x").unwrap_or(hex_string);

    if hex_string.len() % 2 != 0 {
        anyhow::bail!("Hex string must have an even length");
    }

    let mut bytes = Vec::with_capacity(hex_string.len() / 2);

    let mut chars = hex_string.chars();
    while let (Some(a), Some(b)) = (chars.next(), chars.next()) {
        let byte = u8::from_str_radix(&format!("{}{}", a, b), 16)?;
        bytes.push(byte);
    }

    Ok(bytes)
}

fn main() -> anyhow::Result<()> {
    let validated_args = FuelCoreInspectorCliArgs::parse().validate()?;

    let database_handle = DatabaseHandle::from(&validated_args);

    match validated_args.cmd() {
        CommandWithoutConfig::Inspect => {
            let value = database_handle.perform_read(
                validated_args.column(),
                &hex_string_to_bytes(validated_args.key())?,
            )?;
            value.print();
        }
        CommandWithoutConfig::Mutate => {
            database_handle.perform_write(
                validated_args.column(),
                validated_args.key().as_bytes(),
                validated_args.value().as_bytes(),
            )?;
        }
    }

    Ok(())
}
