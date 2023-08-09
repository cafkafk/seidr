<div align="center">

# Seiðr

An experimental Rust GitOps/symlinkfarm orchestrator inspired by GNU Stow.

Highly unstable project, expect each change to be breaking.

[![Built with Nix](https://img.shields.io/badge/Built_With-Nix-5277C3.svg?logo=nixos&labelColor=73C3D5)](https://nixos.org)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md)

[![Unit tests](https://github.com/cafkafk/seidr/actions/workflows/unit-tests.yml/badge.svg)](https://github.com/cafkafk/seidr/actions/workflows/unit-tests.yml)
![Crates.io](https://img.shields.io/crates/v/seidr?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fseidr)
![Crates.io](https://img.shields.io/crates/l/seidr?link=https%3A%2F%2Fgithub.com%2Fcafkafk%2Fseidr%2Fblob%2Fmain%2FLICENCE)

</div>

[![asciicast](https://asciinema.org/a/TVmnEYR3PK40GtoZnwavun0dP.svg)](https://asciinema.org/a/TVmnEYR3PK40GtoZnwavun0dP)

A Rust GitOps/symlinkfarm orchestrator inspired by GNU Stow. Useful for dealing
with "dotfiles", and with git support as a first class feature. Configuration is
done throug a single yaml file, giving it a paradigm that should bring joy to
those that use declarative operating systems and package managers.

Although this isn't really a case where it matters *that* much for performance,
being written in rust instead of e.g. /janky/ scripting languages does also mean
it is snappy and reliable, and the /extensive/ (hardly, but eventually) testing
helps ensure regressions aren't introduced.

That said, we're in 0.Y.Z, *here be dragons* for now (although a little less each
commit).

### Installation

    git clone https://github.com/cafkafk/seidr
    cd seidr
    cargo install --path .

### Configuration
If you want a template, you can copy the file from src/test/config.yaml:

    mkdir -p ~/.config/seidr/
    cp src/test/config.yaml ~/.config/seidr/config.yaml

You should *seriously* change this file before running any commands.

The configuration format will likely break regularly in versions 0.Y.Z.
