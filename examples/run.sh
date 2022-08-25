#!/bin/bash

# build
if [ $RELEASE ]; then
    cargo b -r -q && mkdir lua -p && cp ../target/release/$name_lib.so lua/$name.so -f
    echo "release built"
else
    cargo b -q && mkdir lua -p && cp ../target/debug/$name_lib.so lua/$name.so -f;
    echo "debug built"
fi

# nvim
set_rtp=":set rtp+=$PWD"
nvim -u NONE --headless +"$set_rtp" +":lua $lua" +quit
