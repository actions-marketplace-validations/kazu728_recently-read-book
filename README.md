![alt](https://user-images.githubusercontent.com/34614358/116835980-d72d1c00-abff-11eb-8d17-38ad6fbb6169.png)

# Recently read book :books:

![](https://github.com/kzmat/recently-read/workflows/Update%20recently%20read/badge.svg)

## Overview
Fetch latest read book from booklog and write it to gist.

## Setup

1. Create a new GitHub Gist (https://gist.github.com/)
2. Create a token with the `gist` and `repo` scope and copy it.(https://github.com/settings/tokens/new)

3. Fork this repo
4. Go to the repo **Settings > Secrets**
5. Add the following environment variables.
   - GH_TOKEN: The GitHub token generated above.
   - GIST_ID : The Gist id that are updated.
   - BOOKLOG_USER_ID: Your booklog user id.