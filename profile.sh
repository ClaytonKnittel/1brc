#!/usr/bin/sh

# usage: ./profile.sh <binary> [args...] > onoro.svg

set -e

cargo b --profile profiled
rm -f perf.data
perf record -F 200 --call-graph dwarf -- $@ >/dev/null
perf script | stackcollapse-perf.pl | flamegraph.pl > brc.svg
