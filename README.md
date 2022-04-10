# release-plz-action
action for https://github.com/MarcoIeni/release-plz

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
```

- `fetch-depth: 0` is needed to clone all the git history, which is necessary to
  determine the next version and build the changelog.
