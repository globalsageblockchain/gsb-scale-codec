## Release process

1. Update the version `[workspace.package]` in `Cargo.toml`.
2. Update the version of `gsb-scale-codec-derive` dependency for `gsb-scale-codec` in `Cargo.toml`
   (`gsb-scale-codec-derive = { .., version = "=x.y.z", ...}`).
3. Test the new release on `polkadot-sdk`, ensure `gsb-scale-codec-derive` is also updated when testing.
4. Update the `CHANGELOG.md`.
5. Merge all these changes to master.
6. Add and push a git tag with the version number: `git tag "vX.Y.Z"; git push --tags`.
7. Publish on `crates.io` the package `gsb-scale-codec-derive` and then `gsb-scale-codec`.
