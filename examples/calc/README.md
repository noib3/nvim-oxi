# calc

```lua
local calc = require("calc")

-- All the following commands will print `42` in the Neovim message area.

print(calc.add(1, 41))
print(calc.multiply(2, 21))

print(calc.compute(function(a, b) return a + b; end, 1, 41))
print(calc.compute(function(a, b) return a * b; end, 2, 21))
```
