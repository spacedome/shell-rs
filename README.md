# shell-rs
This is simple shell implementation using the nom parser combinator library.
It has a few builtins `echo, type, exit, cd, pwd` and can call binaries on the PATH with arguments.

This was developed on NixOS, and there is a shell.nix for development,
but due to the way nix handles some nix store paths, and due to the way I resolve paths,
with `canonlicalize` which removes symlinks, certain binaries like ls may behave oddly.
On a typical Linux FSH this probably is not an issue.
