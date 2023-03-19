# ocfs-rs

Rust hooks to do [OCSF](https://ocsf.io) format read and writing.

It's very very very very very early stages, please don't try and use it yet for production 😇

## Crates

There's a couple...

| Name           | Description                                                 | Docs link |
| ---            | ---                                                         | ---       |
| `ocsf-codegen` | Code which generates the `ocsf` module.                     |           |
| `ocsf`         | (Mostly) auto-generated code to do OCSF-parsing and output. | [here](https://yaleman.github.io/ocsf-rs/ocsf/) |

## Getting started

### Cloning the repository and dependencies

This uses git submodules to pull the source schema definitions, so cloning uses extra flags:

```shell
git clone --recurse-submodules https://github.com/yaleman/ocsf-rs
```

If you forgot to do the recurse-submodules thing, or need to change which schema branch you're basing it off, run `make schema_pull` and it'll do the thing. Set an environment variable of `SCHEMA_BRANCH` and you can change it, ie:

```shell
$ SCHEMA_BRANCH=v1.0.0 make schema_pull
git submodule set-branch --branch v1.0.0 "ocsf-schema"
rm -rf "./ocsf-schema"
git submodule update --checkout --force
Submodule path 'ocsf-schema': checked out '8d353b8b2f05be6fe36922f48b15cc40e1b7f400'
git submodule update --remote "ocsf-schema"
echo "Checking version..."
Checking version...
cat "./ocsf-schema/version.json"
{
  "version": "1.0.0-rc.2"
}
```

### Building the ocsf crate

This is all you should need to do:

```shell
make build
```

## Requirements

So far, only rust and `make`. The MSRV's set in `Cargo.toml`.
