# Configuration Reference

Below you can find tables describing all the possible configuration you can do, as well as TOML examples below those tables showing what that configuration would look like.

## General

The `general` [table][toml-table] is for main Hooked configuration.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| config | String | Hooked.toml | The configuration file to use. If your configuration file isn't `Hooked.toml` you should set this accordingly. |
| directory | String | hooks | The directory Hooked looks in for anything related to files. For example: scripts, templates, etc. |
| template | Optional string | | Path to a custom template to be used when installing scripts. See the [install CLI command][cli-install] for more details. |

```toml
[general]
config = "Hooked.toml"
directory = "hooks"
template = "template.sh"
```

## Pre-commit

Pre-commit hooks are defined using `pre_commit` [arrays of tables][toml-arrays-of-tables].

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| name | String | Unnamed Hook | The name of the hook, useful for figuring out which hook failed after it ran. |
| command[^command-and-script] | String | | A command to run when the hook is called. |
| script[^command-and-script] | String | | A script to run when the hook is called. This script should be executable and be located inside the configured general directory. |
| on_failure | String | stop | What to do when the hook task returns a non-zero status code. Can be either "continue" or "stop". |
| git_staged | Optional list of strings | | A list of [globs][globset-docs] that will be checked against staged files. If none of the globs match the hook will be skipped. With no globs defined at all the hook will always run. |

```toml
[[pre_commit]]
name = "Command Example"
command = "echo \"Hey, $USER!\""

[[pre_commit]]
name = "Script Example"
script = "example.sh"
on_failure = "continue"
git_staged = ["*.txt"]
```

## Footnotes

[^command-and-script]: When both a command and script are defined in a hook, *only* the command will be run.

[cli-install]: ../cli/install.md
[globset-docs]: https://docs.rs/globset/0.4.9/globset/#syntax
[toml-table]: https://toml.io/en/v1.0.0#table
[toml-arrays-of-tables]: https://toml.io/en/v1.0.0#array-of-tables
