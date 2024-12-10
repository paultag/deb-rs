# deb: utilities for working with Debian files and formats.

🎉 You found the `deb` crate! 🎉

This crate is under active development, and "soft launched". *Please do
not post widely directing to this crate yet* -- the API shipped today is
unstable, and is likely to change -- fairly significantly -- without much
regard to very precisely following semver until it stabalizes.

You're more than welcome to play with this and use it, but it's not
something I would encourage load bearing infrastructure to be written
with as of right now.

# Introduction

The `deb` crate contains utilities for working with files and formats
commonly found when working with Debian's project tooling, or
infrastructure.

Common use-cases are broken out into modules in the `deb` crate namespace,
such as interacting with [control] files, parsing [dependency]
relationships between Debian packages, parsing and ordering [version]
numbers, or understanding Debian [architecture] strings.

Docs can be found on [docs.rs](https://docs.rs/crate/deb/latest),
and information about the latest release can be found on
[crates.io](https://crates.io/crates/deb).
