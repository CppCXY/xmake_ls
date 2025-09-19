---@meta
-- Minimal xmake utils helpers for typing/completion.

---@class utilslib
utils = {}

--- Ternary-like helper: returns `a` if `cond` truthy else `b`.
---@generic T
---@param cond any
---@param a T
---@param b T
---@return T
function utils.ifelse(cond, a, b) end

--- Generate a random UUID string (e.g. xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx).
---@return string
function utils.uuid() end
