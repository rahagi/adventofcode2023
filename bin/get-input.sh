#!/usr/bin/env bash

[[ -z "$1" ]] && echo "Usage: $0 <day_number>" && exit 1

curl --silent --cookie session="$AOC_SESSION" https://adventofcode.com/2023/day/"${1#0}"/input > ./src/day"$1"/input.txt
