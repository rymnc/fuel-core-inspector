use clap::Parser;
use fuel_core_inspector::{
    cli::{
        CommandWithoutConfig,
        FuelCoreInspectorCliArgs,
    },
    databases::DatabaseHandle,
    printer::PrintHexExt,
};

fn main() -> anyhow::Result<()> {
    let validated_args = FuelCoreInspectorCliArgs::parse().validate()?;

    let database_handle = DatabaseHandle::from(&validated_args);

    match validated_args.cmd() {
        CommandWithoutConfig::Inspect => {
            let value = database_handle
                .perform_read(validated_args.column(), validated_args.key().as_bytes())?;
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
