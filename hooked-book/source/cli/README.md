# CLI Reference

In the following chapters you will find the reference to all of Hooked's CLI subcommands.

## Global Options

These options can be used in all commands.

| Option | Default | Description |
|--------|---------|-------------|
| `-c`, `--config` | Hooked.toml | Path to a Hooked configuration. |
| `-h`, `--help` | | Print help information. |
| `-V`, `--version` | | Print version information. |

## Other

Hooked checks for the [`NO_COLOR` environment variable][no-color] and will disable styling output if present.

[no-color]: https://no-color.org/
