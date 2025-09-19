---@meta
-- Minimal net helpers for xmake scripts (TCP sockets).

---@class netlib
net = {}

---@class net_tcp_opt
---@field timeout? integer # milliseconds

---@class net_tcp
local net_tcp = {}

--- Send raw bytes/string.
---@param data string
---@return integer|nil sent, string? err
function net_tcp:send(data) end

--- Receive bytes up to `n` or until closed.
---@param n? integer
---@return string|nil data, string? err
function net_tcp:recv(n) end

--- Close socket.
function net_tcp:close() end

--- Connect to `host:port` and return tcp socket.
---@param host string
---@param port integer
---@param opt? net_tcp_opt
---@return net_tcp|nil sock, string? err
function net.tcp_connect(host, port, opt) end
