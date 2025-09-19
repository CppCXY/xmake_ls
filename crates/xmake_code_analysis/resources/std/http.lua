---@meta
--
-- Minimal xmake http helpers for static typing/completion.
-- Reference: xmake/core/base/http.lua

---@class httplib
http = {}

---@class http_request_opt
---@field headers? table<string,string>
---@field timeout? integer
---@field insecure? boolean  @allow insecure https
---@field proxy? string

---@class http_response
---@field status integer
---@field headers table<string,string>
---@field body string

--- Perform a HTTP GET request and return response.
---@param url string
---@param opt? http_request_opt
---@return http_response
function http.get(url, opt) end

---@class http_download_opt: http_request_opt
---@field continue? boolean   @resume download if possible

--- Download a file to destination path.
---@param url string
---@param dst string
---@param opt? http_download_opt
---@return boolean ok
function http.download(url, dst, opt) end
