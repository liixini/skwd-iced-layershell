#!/bin/sh
set -eu

repository=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
actual=$(mktemp)
trap 'rm -f "$actual"' EXIT HUP INT TERM

status=0
LC_ALL=C git -C "$repository" grep --untracked -n -w unsafe -- '*.rs' >"$actual" || status=$?
[ "$status" -le 1 ] || exit "$status"

diff -u "$repository/unsafe-baseline.txt" "$actual"
