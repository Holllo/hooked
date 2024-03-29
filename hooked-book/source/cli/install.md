# Install

The `install` command creates the scripts inside `.git/hooks`.

```sh
{{#include ../cli-reference.txt:install}}
```

Below is the default script template that Hooked uses, where `hook_type` is the type of hook to run (like `pre-commit`) and `config_path` is the `general.config` field from the parsed configuration.

```sh
{{#include ../../../hooked-cli/source/templates/default.sh}}
```

You can provide your own template by using the `general.template` configuration setting. If you do, make sure you include a line somewhere that says `# Installed by Hooked.` for the [uninstall CLI command][cli-uninstall].

[cli-uninstall]: ./uninstall.md
