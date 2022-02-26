# Actix Mongodb Jwt Template

> Note: This is personal template used in private project.

# Getting started

+ Copy [.env.default](.env.default) to `.env`
    + Edit `DATABASE_NAME`
+ Edit [Cargo.toml](Cargo.toml)
    + Change package name
      > Also don't forget to change package name in [main.rs](bin/main.rs)
+ run `cargo --run`

## Optional features

| feature           | aaa                                                            |
|-------------------|----------------------------------------------------------------|
| basic-auth        | Handle `Authorization: Basic <token>`                          |
| linux             | enable `io_uring` support  (linux with new kernel only)        |
| static-jwt-secret | link static [jwt secret](jwt_secret) from file into executable |

to enable above feature, just add them to [`default = []`](Cargo.toml)
