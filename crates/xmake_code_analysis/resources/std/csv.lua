---@meta
-- Minimal CSV helpers for xmake scripts.

---@class csvlib
csv = {}

---@class csv_parse_opt
---@field header? boolean # treat first row as header
---@field sep? string # separator, default ","
---@field quote? string # quote char, default '"'

---@class csv_table: table

--- Parse CSV text into Lua table (array of rows or array of dicts if header=true).
---@param s string
---@param opt? csv_parse_opt
---@return csv_table
function csv.parse(s, opt) end

--- Encode Lua table into CSV text.
---@param rows csv_table
---@param opt? csv_parse_opt
---@return string
function csv.encode(rows, opt) end

--- Load CSV file.
---@param path string
---@param opt? csv_parse_opt
---@return csv_table|nil, string? err
function csv.load(path, opt) end

--- Save CSV file.
---@param path string
---@param rows csv_table
---@param opt? csv_parse_opt
---@return boolean ok, string? err
function csv.save(path, rows, opt) end
