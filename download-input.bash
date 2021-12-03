#!/usr/bin/env bash
set -o nounset -o errexit -o pipefail

if [[ -f ./.env ]]; then
    # shellcheck source=./.env
    source ./.env 
fi

mkdir -p "src/inputs/$YEAR"
http get "https://adventofcode.com/$YEAR/day/$DAY/input" "Cookie:session=$SESSION" \
    > "src/inputs/$YEAR/$DAY.txt"
