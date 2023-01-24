![alt](https://user-images.githubusercontent.com/34614358/116835980-d72d1c00-abff-11eb-8d17-38ad6fbb6169.png)

# Recently read book :books:

![](https://github.com/kzmat/recently-read/workflows/Update%20recently%20read/badge.svg)

## Overview

Fetch latest read book from booklog and write it to gist.

## Requirements

- Booklog account https://booklog.jp/
- Gist
- Github token that can update a gist

## Example

```yml
on:
  push:
    branches:
      - master
  schedule:
    - cron: "0 0 * * *"

jobs:
  update-gist:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3
      - name: Update recently read book
        uses: kazu728/recently-read-book@v0.0.1
        env:
          GH_TOKEN: ${{ secrets.GH_TOKEN }}
          GIST_ID: ${{ secrets.GIST_ID }}
          BOOKLOG_USER_ID: ${{ secrets.BOOKLOG_USER_ID }}
```
