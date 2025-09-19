---@meta
--
-- Minimal xmake json helpers for static typing/completion.
-- Reference: xmake/core/base/json.lua

---@class jsonlib
json = {}

---@class json_encode_opt
---@field indent? string|integer
---@field sort_keys? boolean

--- Encode any lua value to json string.
---@param v any
---@param opt? json_encode_opt
---@return string
function json.encode(v, opt) end

---@class json_decode_opt
---@field null? any   @value to map JSON null, default: nil

--- Decode json string to lua value.
---@param s string
---@param opt? json_decode_opt
---@return any
function json.decode(s, opt) end
