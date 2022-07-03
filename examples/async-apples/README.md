# async-apples

Wait a random number of seconds between 4 and 9. Every second sleep a
task and print an update message. If we end before 7 seconds we print how many
apples we have. If not we cancel the task, stop waiting and print a message
saying we're done. Do all of this without blocking the thread.

This crate does the equivalent of the following Lua code

```lua
--[[
async_apples.lua

You can test this inside Neovim by creating a new file with the following code
and calling `:luafile %`.
--]]

local uv = vim.loop

local MAX_WAIT = 7

local UPDATE_MSGS = {
  "Started counting apples!",
  "Counted a few",
  "Still counting",
  "Not quite done yet",
  "Umhh, this might take a while",
  "Not sure if I'll finish in time",
  "Almost done",
}

local WAIT_TIME = math.random(4, 9)

local i = 0

local apple_counter = uv.new_timer()
apple_counter:start(0, 1000, function()
  if i ~= WAIT_TIME then
    print(UPDATE_MSGS[i + 1])
    i = i + 1
    return
  end

  local apples = math.random(0, 100)
  print(("Done in %ss! You have %s apples!"):format(WAIT_TIME, apples))

  apple_counter:stop()
  apple_counter:close()
end)

local stopped = false
local controller = uv.new_timer()
controller:start(0, 1000, function()
  if apple_counter:is_active() then
    if i <= MAX_WAIT then
      return
    end
    apple_counter:stop()
    apple_counter:close()
    stopped = true
  end

  if stopped then
    print("I've had enough of these damn apples!")
  end

  controller:stop()
  controller:close()
end)
```
