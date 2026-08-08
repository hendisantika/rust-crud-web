# rust-crud-web

A small, dependency-light **CRUD** web application written entirely in **Rust**, compiled to
**WebAssembly** and rendered in the browser with [Yew](https://yew.rs) 0.21.

Items are created, listed, searched, updated and removed fully client-side, and the list is
persisted in the browser's `localStorage` — there is no backend to run.

![Rust](https://img.shields.io/badge/rust-2021-orange)
![Yew](https://img.shields.io/badge/yew-0.21-green)
![WebAssembly](https://img.shields.io/badge/wasm-32--unknown--unknown-blue)

## Features

- **Create / Read / Update / Delete** items, each with a name and a price
- **Live search** filtering the table by name
- **Form validation** that reports every invalid field at once (name of at least 2 characters,
  price a number `>= 0`)
- **Persistence** in `localStorage`, so the list survives a page reload
- **Stable ids** derived from the stored data, so ids never collide after a reload
- **Item count and total price** summary
- Styled with [Bulma](https://bulma.io); no JavaScript of our own

## Requirements

| Tool | Version | Notes |
| --- | --- | --- |
| Rust | 1.78+ (stable) | install via [rustup](https://rustup.rs) |
| `wasm32-unknown-unknown` target | — | `rustup target add wasm32-unknown-unknown` |
| [Trunk](https://trunkrs.dev) | 0.21+ | `cargo install trunk --locked` |

`rust-toolchain.toml` already requests the wasm target plus `clippy` and `rustfmt`, so rustup
installs them for you on the first build.

> Trunk 0.21.14+ needs rustc 1.81 or newer. On an older toolchain either install a matching
> Trunk (`cargo install trunk --version 0.21.8 --locked`) or grab a prebuilt binary from the
> [Trunk releases](https://github.com/trunk-rs/trunk/releases) page.

## Running

```bash
# development server with auto-rebuild on http://127.0.0.1:8080
trunk serve

# optimised production build, written to ./dist
trunk build --release
```

`dist/` is a folder of static files — serve it with any web server, or publish it to GitHub
Pages, Netlify, and friends. If it is not hosted at the domain root, build with the correct base
path: `trunk build --release --public-url /rust-crud-web/`.

## Testing and linting

```bash
cargo test                                             # validation unit tests
cargo fmt --check                                      # formatting
cargo clippy --target wasm32-unknown-unknown --all-targets   # lints
```

The same three commands, plus a release build, run in CI on every push and pull request
(`.github/workflows/ci.yml`).

## Project layout

```
src/
├── main.rs    entry point; mounts the root component
├── model.rs   root component: owns the item list, search state and localStorage access
├── modal.rs   create/update dialog with validation feedback
├── input.rs   reusable controlled text input
└── item.rs    Item data model, form validation and its unit tests
index.html     Trunk entry point (Bulma + ionicons via CDN)
Trunk.toml     build/serve configuration
```

### How the state flows

`Model` is the single owner of the data. It renders `Modal` only while a record is being created
or edited, and hands it the item to work on. `Modal` keeps a private draft of the form fields,
validates it on submit, and emits either `on_save` with a valid `Item` or renders the collected
errors. `Model` then assigns an id (for new items), writes the whole list to `localStorage`, and
re-renders the table. Rows are addressed by **id** rather than by index, so editing or removing
stays correct while the list is filtered by the search box.

Stored data lives under the `yew.rust.crud.database` key. Clearing your browser's site data
resets the application.

## Note on `implicit-clone`

`Cargo.toml` pins `implicit-clone = "=0.4.1"` on purpose: version 0.4.2 and later build against a
different `indexmap` major version than Yew 0.21 does, which makes the build fail with a
`IMap: From<IndexMap>` trait error. Keep the pin until the project moves to a newer Yew.

## License

MIT © [Hendi Santika](https://github.com/hendisantika)
