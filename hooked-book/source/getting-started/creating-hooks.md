# Creating hooks

All of Hooked's configuration is done inside the `Hooked.toml` file. It should be placed inside the same directory your `.git` directory is.

```
your-awesome-project
├── Hooked.toml      <- !
├── .git             <- !
│   └── ...
├── source
│   └── ...
└── ...
```

Inside `Hooked.toml`, hooks are defined as [arrays of tables][toml-arrays-of-tables] with a name (optional but highly encouraged) and either a command or script.

A simple pre-commit hook is defined like the following:

```toml
[[pre_commit]]
name = "Hook name, optional but highly encouraged"
command = "echo \"Hey, $USER!\""
```

To test that this works, use the [run CLI command][cli-run] and you should see something like this.

```sh
$ hooked run pre-commit
Hooked: Running 1 pre-commit hook.
	✓ Hook name, optional but highly encouraged
```

> To see what happens when a command fails, append `&& exit 1` to your configured command and run it again.

If you want to run a script, create a `hooks` directory and place your script there. Then in your configuration file use the "script" field for your hook. Make sure that your script is executable!

```diff
[[pre_commit]]
name = "Hook name, optional but highly encouraged"
command = "cargo test"
+
+ [[pre_commit]]
+ name = "A script hook"
+ script = "run-tests.sh"
```

> Defining both a command and a script for a hook will make *only* the command run.

The directory Hooked looks in can be configured via the `general.directory` field, should you wish to change it.

```diff
+ [general]
+ # Use the same directory where Hooked.toml is.
+ directory = "."
+
[[pre_commit]]
name = "Hook name, optional but highly encouraged"
command = "echo \"Hey, $USER!\""

[[pre_commit]]
name = "A script hook"
script = "run-tests.sh"
```

---

For a full list of everything you can configure in `Hooked.toml` see the [Configuration Reference][configuration]. And for the CLI there is the [CLI Reference][cli].

Those are the basics of creating hooks! Now to make Git use these hooks continue on to the next chapter.

[cli]: ../cli/
[cli-run]: ../cli/run.md
[configuration]: ../configuration/
[toml-arrays-of-tables]: https://toml.io/en/v1.0.0#array-of-tables
