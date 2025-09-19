---@meta
-- Minimal base64 helpers for typing/completion.

---@class base64lib
base64 = {}

--- Encode bytes or string to base64 text.
---@param s string
---@return string
function base64.encode(s) end

--- Decode base64 text to raw string bytes.
---@param s string
---@return string
function base64.decode(s) end
