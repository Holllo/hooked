# Uninstall

The `uninstall` command removes script files inside `.git/hooks`.

```sh
hooked uninstall --help
Remove installed hooks

Usage: hooked uninstall [OPTIONS]

Options:
      --all              Remove hooks not installed by Hooked
  -c, --config <CONFIG>  Path to a Hooked configuration [default: Hooked.toml]
  -h, --help             Print help information
  -V, --version          Print version information
```

By default Hooked will only remove scripts that have a `# Installed by Hooked.` line in them, using `--all` however will remove all script files.
