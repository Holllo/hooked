# Configuration Reference

Below you can find tables describing all the possible configuration you can do, as well as TOML examples below those tables showing what that configuration would look like.

## General

The `general` [table][toml-table] is for main Hooked configuration.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| config | String | Hooked.toml | The configuration file to use. If your configuration file isn't `Hooked.toml` you should set this accordingly. |
| directory | String | hooks | The directory Hooked looks in for anything related to files. For example: scripts, templates, etc. |

```toml
[general]
config = "Hooked.toml"
directory = "hooks"
```

## Pre-commit

Pre-commit hooks are defined using `pre_commit` [arrays of tables][toml-arrays-of-tables].

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| name | String | Unnamed Hook | The name of the hook, useful for figuring out which hook failed after it ran. |
| command[^command-and-script] | String | | A command to run when the hook is called. |
| script[^command-and-script] | String | | A script to run when the hook is called. This script should be executable and be located inside the configured general directory. |
| on_failure | String | stop | What to do when the hook task returns a non-zero status code. Can be either "continue" or "stop". |

```toml
[[pre_commit]]
name = "Command Example"
command = "echo \"Hey, $USER!\""

[[pre_commit]]
name = "Script Example"
script = "example.sh"
on_failure = "continue"
```

## Footnotes

[^command-and-script]: When both a command and script are defined in a hook, *only* the command will be run.

[toml-table]: https://toml.io/en/v1.0.0#table
[toml-arrays-of-tables]: https://toml.io/en/v1.0.0#array-of-tables
