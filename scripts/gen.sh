#!/usr/bin/env bash
set -euo pipefail

DAY=${1-}
[ -n "$DAY" ] || (echo "ERROR: specify the day (e.g., dec23)" && exit 1)

echo -n "Create $DAY module? [Y|n] "
read -r ans
case $ans in
  y | Y | "")
    echo "Generating..."
    ;;
  *)
    exit 1
    ;;
esac

SOURCE=$(dirname "${BASH_SOURCE[0]}")
DEST="${SOURCE}/../src/${DAY}"

mkdir "$DEST"
touch "${DEST}/example_1.txt"
touch "${DEST}/input_1.txt"
sed "s/__DAY__/${DAY}/" "${SOURCE}/mod.rs.tpl" > "${DEST}/mod.rs"
echo "mod ${DAY};" >> "${SOURCE}/../src/lib.rs"

echo "Done."