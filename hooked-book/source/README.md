# Hooked 🪝

```toml
# Hooked.toml
[[pre_commit]]
name = "Cargo Check"
command = "cargo check"
```

```sh
# Install Hooked to ".git/hooks".
$ hooked install

# Commit away!
$ git commit -m 'Start using Hooked!'
Hooked: Running 1 pre-commit hook.
	✓ Cargo Check
[main b386e59] Start using Hooked!
 1 file changed, 3 insertions(+)
 create mode 100644 Hooked.toml
```

Hooked is a manager for [Git hooks][git-hooks]. It is language-agnostic (not built for any specific language or ecosystem), has a low barrier to entry (hooks can be one-liners in a configuration file) and is extensible (most of what Hooked does can be replaced through its configuration).

If you've been convinced and would like to give it a shot, [Getting Started][getting-started] is the place to be.

[getting-started]: ./getting-started/
[git-hooks]: https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks
[internals]: ./internals/
