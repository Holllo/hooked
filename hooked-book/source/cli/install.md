# Install

The `install` command creates the scripts inside `.git/hooks`.

```sh
$ hooked install --help
Install Hooked into ".git/hooks"

Usage: hooked install [OPTIONS]

Options:
      --overwrite        Overwrite existing files
  -c, --config <CONFIG>  Path to a Hooked configuration [default: Hooked.toml]
  -h, --help             Print help information
  -V, --version          Print version information
```

Below is the default script template that Hooked uses, where `hook_type` is the type of hook to run (like `pre-commit`) and `config_path` is the `general.config` field from the parsed configuration.

```sh
{{#include ../../../hooked-cli/source/templates/default.sh}}
```
