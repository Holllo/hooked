# The Git side of things

In the previous chapter you manually tested hooks using the [CLI `run` command][cli-run]. Now to install those hooks and have Git use them, we'll use the [CLI `install` command][cli-install].

```sh
hooked install
```

What this command does is create the scripts inside the `.git/hooks` directory that Git will run whenever you do Git stuffs. By default, these scripts are a simple call to the [CLI `run` command][cli-run].

So unless you change anything in the hook templates, you only need to run this once, as it will read your current configuration any time those scripts get called.

## Uninstalling hooks

If at any point you want to remove the scripts from your `.git/hooks` directory, use the [CLI `uninstall` command][cli-uninstall].

```sh
hooked uninstall
```

[cli-install]: ../cli/install.md
[cli-run]: ../cli/run.md
[cli-uninstall]: ../cli/uninstall.md
[templating]: ../configuration/templating/
