# Microprocessor Architecture Lab - Contributing guide

## Adding a new lab

In order to add a new lab, you will need to generate the crate using `cargo generate`, based on the template found [here](https://github.com/UPB-FILS-MA/lab-template).

> If you would like to change anything at the template, please open a Pull Request there.

### Prerequisites

**cargo-generate** installation:

```shell
cargo install cargo-generate
```

### Usage

#### Generate the crate

```shell
cargo generate --git https://github.com/UPB-FILS-MA/lab-template.git --branch <DESIRED-BRANCH>
```

#### Add it to the workspace

Add crate as member in the workspace's `Cargo.toml`.
