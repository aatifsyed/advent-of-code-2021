#!/usr/bin/env bash
set -o nounset -o errexit -o pipefail

which http >/dev/null || { echo "install https://httpie.io/cli"; exit 1; }

if [[ -f ./.env ]]; then
    # shellcheck source=./.env
    source ./.env 
fi

_=${SESSION:?"Set to your AOC in-browser cookie. You can save it in ./.env"}

mkdir -p "src/inputs/$YEAR"
http get "https://adventofcode.com/$YEAR/day/$DAY/input" "Cookie:session=$SESSION" \
    > "src/inputs/$YEAR/$DAY.txt"
