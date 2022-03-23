# Actix Mongodb Jwt Template

> Note: This is personal template used in private project.

# Getting started

## General method

+ Copy [.env.default](.env.default) to `.env`
    + Edit `DATABASE_NAME`
+ Edit [Cargo.toml](Cargo.toml)
    + Change package name
      > Also don't forget to change package name in [main.rs](bin/main.rs)
+ run `cargo --run`

## Via docker

By default, it will build fresh image every build (not cached), or you can modify [`Dockerfike`](Dockerfile)
and [`docker-compose.yml`](docker-compose.yml) to fit your requirement

Note: you don't need `.env` because you can edit in `docker-compose.yml` (environment section)
```shell
docker compose up --build
# --build to ensure image is rebuild every time you run this command
```

## Optional features

| feature           | aaa                                                            |
|-------------------|----------------------------------------------------------------|
| basic-auth        | Handle `Authorization: Basic <token>`                          |
| linux             | enable `io_uring` support  (linux with new kernel only)        |
| static-jwt-secret | link static [jwt secret](jwt_secret) from file into executable |

to enable above feature, just add them to [`default = []`](Cargo.toml)
