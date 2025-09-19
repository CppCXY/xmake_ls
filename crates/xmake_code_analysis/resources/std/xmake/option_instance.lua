---@meta
---TODO: this is not complete
---[option_instance](https://xmake.io/api/scripts/option-instance)

---@class Option
local Option = {}

---
---Get the name of the option
---
---[Open in browser](https://xmake.io/api/scripts/option-instance#option-name)
--- Example:
--- ```lua
--- option("simd")
---     set_showmenu(true)
---     on_check(function (opt)
---         print("checking option:", opt:name())
---     end)
--- option_end()
--- ```
---
---@return string
function Option:name() end

---
---Get the values of the option by name
---
---[Open in browser](https://xmake.io/api/scripts/option-instance#option-get)
--- Example:
--- ```lua
--- on_check(function (opt)
---     local v = opt:get("defines") -- string
---     if v then
---         print("defines:", v)
---     end
--- end)
--- ```
---
---@param key string Option key
---@return string
function Option:get(key) end

---
---Set the values of the option by name (If you just want to add values use [option:add](#optionadd))
---
---[Open in browser](https://xmake.io/api/scripts/option-instance#option-set)
--- Example:
--- ```lua
--- on_check(function (opt)
---     opt:set("enabled", true)
--- end)
--- ```
---
---@param key string Option key
---@param value string Option value
---@return nil
function Option:set(key, value) end

---
---Add to the values of the option by name
---
---[Open in browser](https://xmake.io/api/scripts/option-instance#option-add)
--- Example:
--- ```lua
--- on_check(function (opt)
---     opt:add("defines", "HAVE_SIMD")
--- end)
--- ```
---
---@param key string Option key
---@param value string Option value
---@return nil
function Option:add(key, value) end

---
---Return depending option
---
---[Open in browser](https://xmake.io/api/scripts/option-instance#option-dep)
--- Example:
--- ```lua
--- option("with_ssl")
---     add_deps("openssl")
---     on_check(function (opt)
---         local d = opt:dep("openssl")
---         if d and d:enabled() then
---             print("openssl dependency enabled")
---         end
---     end)
--- option_end()
--- ```
---
---@param name string Depending option name
---@return Option|nil
function Option:dep(name) end

---
---Check whether this option is enabled
---
---[Open in browser](https://xmake.io/api/scripts/option-instance#option-enabled)
---
---@return boolean
function Option:enabled() end

---
---Enable/Disable this option
---
---[Open in browser](https://xmake.io/api/scripts/option-instance#option-enable)
--- Example:
--- ```lua
--- -- Enable or disable an option programmatically
--- if not option:enabled() then option:enable(true) end
--- ```
---
---@param enabled boolean Whether to enable
---@return nil
function Option:enable(enabled) end

---
---Get all depending options
---
---[Open in browser](https://xmake.io/api/scripts/option-instance#option-deps)
--- Example:
--- ```lua
--- on_check(function (opt)
---     for name, dep in pairs(opt:deps()) do
---         print("dep:", name, dep:enabled())
---     end
--- end)
--- ```
---
---@return { [string]: Option }
function Option:deps() end
