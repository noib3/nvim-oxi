#!/bin/bash

# name
project=${project:-${PWD##*/}}
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    name_lib=${name_lib:-"lib$project.so"}
elif [[ "$OSTYPE" == "darwin"* ]]; then
    name_lib=${name_lib:-"lib$project.dylib"}
else
    >&2 echo "Not supported"
fi
name=${name:-"$project.so"}

# build
features=${features:-""}
if [ $RELEASE ]; then
    cargo b -r -q $features && mkdir lua -p && cp ../target/release/$name_lib lua/$name -f;
    echo "release built"
else
    cargo b -q $features && mkdir lua -p && cp ../target/debug/$name_lib lua/$name -f;
    echo "debug built"
fi

# nvim
set_rtp=":set rtp+=$PWD"
nvim -u NONE --headless +"$set_rtp" +":lua $lua" +quit
