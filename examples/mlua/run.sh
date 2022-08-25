#!/bin/bash
export name="lua"
export name_lib="liblua"
export lua="
require'lua'.greetings()
"
bash ../run.sh
