---@meta
-- Minimal process helpers matching common xmake usage.

---@class processlib
process = {}

---@class process_opt
---@field env? table<string,string>
---@field cwd? string
---@field stdout? 'pipe'|'inherit'|string
---@field stderr? 'pipe'|'inherit'|string
---@field stdin? 'pipe'|'inherit'|string
---@field detach? boolean

---@class process_handle
local process_handle = {}

--- Read all stdout content (if piped).
---@return string|nil, string? err
function process_handle:read() end

--- Read all stderr content (if piped).
---@return string|nil, string? err
function process_handle:readerr() end

--- Wait for process to exit and return code.
---@return integer|nil code, string? err
function process_handle:wait() end

--- Kill the process.
---@param signal? integer|string
---@return boolean ok, string? err
function process_handle:kill(signal) end

--- Close process handle and related pipes.
---@return boolean ok, string? err
function process_handle:close() end

--- Spawn a process with command string.
---@param cmd string
---@param opt? process_opt
---@return process_handle|nil, string? err
function process.open(cmd, opt) end

--- Spawn a process with argv vector.
---@param program string
---@param argv string[]
---@param opt? process_opt
---@return process_handle|nil, string? err
function process.openv(program, argv, opt) end
