#!/usr/bin/sh

# usage: ./profile.sh <binary> [args...] > onoro.svg

rm perf.data
perf record -g --call-graph dwarf,16384 -- $@ >/dev/null
perf script -F comm,pid,tid,time,event,ip,sym,dso,trace | stackcollapse-perf.pl | flamegraph.pl > brc.svg
