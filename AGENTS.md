# nju-cli

This is a cli to interact NanJing University online sites.

## Project structure

This is a multi-crate rust project, managed by nix and cargo. Each crate correspond to an nju site, with a few exceptions like `cli` and `common`.

## Environment

For Unix-like systems, use nix devshell.

For Windows, you're on your own.

## Coding guidelines

- Use reqwest for HTTP requests. Each lib crate should take a reqwest client as argument (instead of creating new session every time)
- Use anyhow for error handling
- Use chinese for comments, as this aligns with NJU sites.
- It's important to keep code simple. Use external dependencies if it helps doing chores.
