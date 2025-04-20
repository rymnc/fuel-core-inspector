# Fuel Core Inspector

Inspect and mutate database k-v pairs in Fuel Core.

Supports column validation.

## Installation

### With `cargo`

```bash
cargo install --git https://github.com/rymnc/fuel-core-inspector --locked
```

### Prebuilt binaries

Head over to the [nightly release](https://github.com/rymnc/fuel-core-inspector/releases/tag/nightly) and download the appropriate binary for your platform.

## Usage

### Inspect Command

Retrieve key-value pairs from the database:

```bash
fuel-core-inspector inspect --database <DATABASE_NAME> --path <PATH_TO_DATABASE> --column <COLUMN_NAME> --key <KEY>
```

### Mutate Command

Modify key-value pairs in the database:

```bash
fuel-core-inspector mutate --database <DATABASE_NAME> --path <PATH_TO_DATABASE> --column <COLUMN_NAME> --key <KEY> --value <NEW_VALUE>
```

### Arguments

- `--database`: Specifies the database name
- `--path`: Path to the database
- `--column`, `-c`: Column name
- `--key`, `-k`: Key to inspect or mutate
- `--value`, `-v`: Value to write (only required for mutate commands)

## Examples

### Get help

```bash
fuel-core-inspector --help
```

### Inspecting a Value

```bash
fuel-core-inspector inspect --database fuel_core --path /var/data/fuel --column metadata --key block_height
```

### Mutating a Value

> [!WARNING]
> there be dragons here. make sure the value you pass is already serialized

```bash
fuel-core-inspector mutate --database fuel_core --path /var/data/fuel --column metadata --key sync_status --value completed
```


## TODOS

1. Key serializer
2. Value deserializer
