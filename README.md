# battleship-rs

A battleship game written in Rust.

## roadmap

1. A basic implementation of battleship that can be played in the CLI. :white_check_mark:
2. A basic implementation of battleship that can be played on the web. :white_check_mark:
3. Add Vue 3 + Typescript + Vite on the web. :white_check_mark:
4. Create player profile web application: user auth, account management.
5. Add multiplayer functions to web app: user auth, lobbies, invites, stat saves.
6. Add online functions to CLI: user auth, offline mode, local stat saves.
7. Multiplayer match making and ranking system.

## what?

This repo will contain the code for running battleship and playing battleship either through a CLI or the web.

### technologies

- Rust
- Vue 3
- Nuxt
- SQL
- Redis
- REST
- WebSockets
- Github Actions

## why?

For fun and to learn a few things:

- General Rust code
- Wasm in Rust
- Use React with Wasm + Rust
- CLI in Rust
- Structuring of Rust projects
- Practice OOP in Rust
- Unit testing in Rust
- Hosting Wasm apps in the cloud
- Custom user auth
- Some distributed system fun

## how?

### engine

_under construction_

### cli

_under construction_

### web

_under construction_

### automation

To validate the GitHub Actions locally you can use `act`.

```bash
act -W 'WORKFLOW_NAME' -P ubuntu-latest=ghcr.io/catthehacker/ubuntu:rust-latest
```
