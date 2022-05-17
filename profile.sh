#!/usr/bin/env bash

# http://www.codeofview.com/fix-rs/2017/01/24/how-to-optimize-rust-programs-on-linux/

valgrind \
    --tool=callgrind \
    --dump-instr=yes \
    --collect-jumps=yes \
    --simulate-cache=yes \
    target/release/csv_validator \
    --csv-file examples/assortment_100.csv \
    --constraints-file examples/constraints.json
