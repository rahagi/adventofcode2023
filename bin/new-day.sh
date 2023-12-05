#!/usr/bin/env bash

[[ -z "$1" ]] && echo "Usage: $0 <day_number>" && exit 1

cp -r ./template/day ./src/day"$1"
find ./src/day"$1" -type f -exec sed -i -e "s/{{DAYNUM}}/$1/g" {} \;
echo "pub mod day$1;" >> ./src/lib.rs
