---@meta
--
-- Minimal xmake hash helpers for static typing/completion.
-- Reference: xmake/core/base/hash.lua

---@class hashlib
hash = {}

--- Calculate MD5 of a string and return hex digest.
---@param s string
---@return string
function hash.md5(s) end

--- Calculate SHA256 of a string and return hex digest.
---@param s string
---@return string
function hash.sha256(s) end

--- Calculate SHA1 of a string and return hex digest.
---@param s string
---@return string
function hash.sha1(s) end
