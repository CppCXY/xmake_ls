---@meta
---TODO: this is not complete
---[option_instance](https://xmake.io/#/manual/option_instance)

---@class Option
local Option = {}

---
---Get the name of the option
---
---[Open in browser](https://xmake.io/#/manual/option_instance?id=optionname)
---
---@return string
function Option:name() end

---
---Get the values of the option by name
---
---[Open in browser](https://xmake.io/#/manual/option_instance?id=optionget)
---
---@param key string Option key
---@return string
function Option:get(key) end

---
---Set the values of the option by name (If you just want to add values use [option:add](#optionadd))
---
---[Open in browser](https://xmake.io/#/manual/option_instance?id=optionset)
---
---@param key string Option key
---@param value string Option value
---@return nil
function Option:set(key, value) end

---
---Add to the values of the option by name
---
---[Open in browser](https://xmake.io/#/manual/option_instance?id=optionadd)
---
---@param key string Option key
---@param value string Option value
---@return nil
function Option:add(key, value) end

---
---Return depending option
---
---[Open in browser](https://xmake.io/#/manual/option_instance?id=optiondep)
---
---@param name string Depending option name
---@return Option|nil
function Option:dep(name) end

---
---Check whether this option is enabled
---
---[Open in browser](https://xmake.io/#/manual/option_instance?id=optionenabled)
---
---@return boolean
function Option:enabled() end

---
---Enable/Disable this option
---
---[Open in browser](https://xmake.io/#/manual/option_instance?id=optionenable)
---
---@param enabled boolean Whether to enable
function Option:enable(enabled) end
