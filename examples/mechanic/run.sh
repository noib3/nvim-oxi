#!/bin/bash
export lua='
local mechanic = require("mechanic")

local fixed = mechanic.fix({
  manufacturer = "Tesla",
  miles = 69420,
  works = false,
  problem = "kills_pedestrians",
})

assert(fixed.works)
assert(fixed.problem == nil)
print(vim.inspect(fixed))
'
bash ../run.sh
