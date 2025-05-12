# MILITARY UNIT MANAGEMENT SYSTEM v2.0

<p><span style="color:red">IN DEVELOPMENT</span></p>

## Description
Management system written in [Rust](https://www.rust-lang.org) with 
[WASM](https://developer.mozilla.org/en-US/docs/WebAssembly) `UI`
and [cross-platform](https://en.wikipedia.org/wiki/Cross-platform) application
based on microservice's architecture.

## Components 

>**API-SERVER** `back-end` \
**standalone server without additional dependencies**
>>[Axum](https://axum.rs) - modular web framework built with Tokio, Tower, and Hyper \
>>[SurrealDB](https://surrealdb.com) - no-sql `embedded`, `local` or `remote` database

>**UI** `front-end` \
**user interface based on `WASM`**
>>[Dioxus](https://dioxuslabs.com) - is the Rust framework for building fullstack web, desktop, and mobile apps \
>>[Tailwind](https://tailwindcss.com) - CSS framework for rapid UI development \
>>[DaisyUI](https://daisyui.com) - open source **Tailwind CSS component library**

>**APPLICATION** `windows` `android` etc. \
**cross-platform application**
>>[Tauri **v2.0**](https://v2.tauri.app) - cross-platform app framework written in Rust

## Docker compose
- [x] SurrealDB - `TiKV cluster` no-sql database 
- [x] RabbitMQ - `message broker` microservices messaging
- [x] Prometheus - `monitoring` metrics
- [x] Grafana - `monitoring` visualisation
- [x] Caddy - `reverse proxy` https-to-http
- [x] access - `api access` microservice `entire access-point`
- [x] logger - `logging` microservice
- [x] auth - `authorization` microservice
 
## Getting started

### Prerequirements

>[LLVM](https://llvm.org) -OR- [Microsoft build tools](https://visualstudio.microsoft.com) - The LLVM Compiler Infrastructure \
>[CMAKE](https://cmake.org) - cross-platform, open-source build automation tool
>[Docker](https://www.docker.com/) - containerization platform \
>[mkcert](https://github.com/FiloSottile/mkcert) - simple tool for creating local trusted development certificates

1. Make localhost certificate for development purposes with [mkcert](https://github.com/FiloSottile/mkcert)
```bash
mkcert -install
mkcert localhost
```
2. Rename `localhost.pem` to `cert.pem` and `key.pem` to `private.key` and place them in `./cfg/certificates` folder

3. Install cargo make crate
```bash
cargo install --force cargo-make
```

### Make commands

* Install dependencies for cross-compilation 
```bash
cargo make install-cross
```

* Build entire server API microservices
```bash
cargo make build-api
```

* Compose and run a server with all dependencies
```bash
cargo make compose-up
```

* Build a single service
```bash
cargo make build-'service'
```

* Compose and rerun a single service
```bash
cargo make compose-'service'
```

* Build, compose, and rerun a single service
```bash
cargo make 'service'
```

* Remove all Docker containers 
```bash
cargo make compose-down
```

* Remove all Docker unused volumes
```bash
cargo make remove-volumes
```

### Development...