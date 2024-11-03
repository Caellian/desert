# Desert

Desert is a fork of `resvg`, `usvg`, `svgtypes` and `simplecss` with purpose of
adding deferred evaluation to those crates.

It acheives this by making `usvg` produce a Tree which can be configured after
parsing.

At some point, it might be split into PRs and merged back into upstream
repositories, but these changes are still largely experimental and untested, so
I made this monorepo to simplify experimentation.

The name comes from adding Es to "Dynamic SVG Rendering Toolkit" acronym (DSRT)
to make it easier to pronounce.

# License

`resvg` and `usvg` are licensed under [MPLv2](./LICENSE-MPL)
license, though it's [being relicensed to Apache 2.0/MIT](https://github.com/linebender/resvg/issues/838).

`simplecss` and `svgtypes` are licensed under dual
[Apache 2.0](./crates/svgtypes/LICENSE-APACHE) and [MIT](./crates/svgtypes/LICENSE-MIT)
license.

Any work done by me after
[`84c7b0cb`](https://github.com/Caellian/desert/commit/84c7b0cbe1e9e643737d6a2aafd9720ee6958642)
is licensed under dual [Apache 2.0](./LICENSE-APACHE) and [MIT](./LICENSE-MIT)
license.
