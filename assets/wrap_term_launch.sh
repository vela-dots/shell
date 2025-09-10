#!/usr/bin/env sh

cat ~/.local/state/vela/sequences.txt 2>/dev/null

exec "$@"