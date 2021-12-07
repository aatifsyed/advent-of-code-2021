#!/usr/bin/env bash
set -o nounset -o errexit -o pipefail

./download_input.bash

LOCATION="./src/$NAME.rs"
printf -- "- %04d, day %02d: [%s](%s)" "$YEAR" "$DAY" "$NAME" "$LOCATION" >> ./README.md
sort --output ./README.md ./README.md
touch "$LOCATION"
printf "mod %s;\n" "$NAME" >> ./src/lib.rs
