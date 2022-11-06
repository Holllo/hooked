# Uninstall

The `uninstall` command removes script files inside `.git/hooks`.

```sh
{{#include ../cli-reference.txt:uninstall}}
```

By default Hooked will only remove scripts that have a `# Installed by Hooked.` line in them, using `--all` however will remove all script files.
