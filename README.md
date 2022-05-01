# release-plz-action

Action for https://github.com/MarcoIeni/release-plz

## Input variables

- `release-pr`: Run the release-pr command. (Default: `"true"`)
- `release`: Run the release command. (Default: `"true"`)
- `registry`: Registry where the packages are stored. The registry name needs to be present in the Cargo config. If unspecified, crates.io is used. (Defaults to crates.io).
- `no_changelog`: Don't create changelog. (Default: `"false"`).
- `project_manifest`: Path to the Cargo.toml of the project you want to update. Both Cargo workspaces and single packages are supported. (Defaults to the root directory).
- `args`: Release-plz additional arguments. (Default: `""`)

## Example

```yaml
name: Release-plz

on:
  push:
    branches:
      - main

jobs:
  release-plz:
    name: Release-plz
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v3
        with:
          fetch-depth: 0
      - name: Run release-plz
        uses: MarcoIeni/release-plz-action@main
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

- `fetch-depth: 0` is needed to clone all the git history, which is necessary to
  determine the next version and build the changelog.
