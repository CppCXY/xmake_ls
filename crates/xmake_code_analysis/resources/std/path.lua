---@meta
-- Copyright (c) 2018. tangzx(love.tangzx@qq.com)
--
-- Licensed under the Apache License, Version 2.0 (the "License"); you may not
-- use this file except in compliance with the License. You may obtain a copy of
-- the License at
--
-- http://www.apache.org/licenses/LICENSE-2.0
--
-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
-- WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
-- License for the specific language governing permissions and limitations under
--- Options for `path.translate`/`pathobj:translate`.
---@class path_translate_opt
---@field normalize? boolean  @Also normalize while translating
---@field separator? string   @Target separator, e.g. "/" or "\\"

-- the License.

---
--- Path utilities provided by xmake. Cross-platform helpers for
--- joining, normalizing, translating paths and handling PATH-like
--- environment variables.
---@param p string
---@return string
--- Reference: xmake/core/base/path.lua
---@class pathlib
---@overload fun(p:string, transform?:fun(p:string):string): pathobj
---@param p string
---@param opt? path_translate_opt
---@return string
path = {}

---@class pathobj
---@param p string
---@param sep? string
---@return string
local pathobj = {}

--====================
---@param p string
---@param rootdir? string
---@return string
-- Module functions (path.*)
--====================

---@param p string
---@param rootdir? string
---@return string
--- Normalize a path: collapse './', '..', duplicate separators
--- and format separators for the current platform.
function path.normalize(p) end
---@param p string
---@return boolean

--- Translate a path. If `opt.normalize` is true, it will also normalize.
function path.translate(p, opt) end
---@param p string
---@return string

--- Get the directory part of a path.
function path.directory(p, sep) end
---@param p string
---@return string

--- Get absolute path (relative to `rootdir`).
function path.absolute(p, rootdir) end
---@param p string
---@return string

--- Get relative path (relative to `rootdir`).
function path.relative(p, rootdir) end
---@param p string
---@param sep? string
---@return string

--- Check whether path is absolute.
function path.is_absolute(p) end
---@param p string
---@return string

--- Convert to unix-style separators (especially on Windows).
function path.unix(p) end

---@param p string
---@param level? integer
---@return string|nil
--- Convert to Cygwin style path (e.g. `C:\` -> `/c/`).
function path.cygwin(p) end

---@param p string
---@param ... string
---@return string
--- Convert to Msys/Cygwin usable path (e.g. "c:\\xx" -> "/c/xx").
function path.cygwin_path(p) end

---@param p string
---@return string[]
--- Get filename without directory.
function path.filename(p, sep) end

---@return string
--- Get basename without extension.
function path.basename(p) end

---@return string
--- Get extension (with dot). When `level>1`, return multi-level
--- combined extensions.
function path.extension(p, level) end
---@param env_path string
---@return string[]

--- Join path segments with proper separators and translation.
function path.join(p, ...) end
---@param paths string[]
---@param envsep? string
---@return string

--- Split path by separators.
function path.split(p) end
---@param p string
---@return boolean

--- Get the platform path separator ('/' or '\\\\').
function path.sep() end
---@param pattern string
---@return string

--- Get the environment PATH separator (':' or ';').
function path.envsep() end
---@param p string
---@param transform? fun(p:string):string
---@return pathobj

--- Split a PATH-like environment variable. Handles quotes/flags safely.
function path.splitenv(env_path) end
---@param p any
---@return boolean

--- Join a list into a PATH-like environment variable safely.
function path.joinenv(paths, envsep) end

---@return string
--- Check if the last character is a separator.
function path.islastsep(p) end

---@return string
--- Convert a glob pattern to a Lua pattern (supports '*' and '**').
function path.pattern(pattern) end

---@param p string
---@return nil
--- Create a new path instance (same as calling `path(p, transform)`).
function path.new(p, transform) end

---@return boolean
--- Check whether a value is a path instance.
function path.instance_of(p) end

---@param transform fun(p:string):string
---@return nil

--- Get the display string (after applying transform).
function pathobj:str() end
---@return fun(p:string):string|nil

--- Get the raw string.
function pathobj:rawstr() end
---@return pathobj

--- Set the raw string.
function pathobj:set(p) end
---@return pathobj

--- Check whether empty.
function pathobj:empty() end
---@param opt? path_translate_opt
---@return pathobj

--- Set the transform function.
function pathobj:transform_set(transform) end
---@return pathobj

--- Get the transform function.
function pathobj:transform_get() end
---@return string

--- Clone this instance.
function pathobj:clone() end
---@return string

--- Normalize (return a new instance).
function pathobj:normalize() end
---@param level? integer
---@return string|nil

--- Translate (return a new instance).
function pathobj:translate(opt) end
---@return pathobj

--- Convert to unix style (return a new instance).
function pathobj:unix() end
---@param rootdir? string
---@return pathobj

--- Filename.
function pathobj:filename() end
---@param rootdir? string
---@return pathobj

--- Basename.
function pathobj:basename() end
---@param ... string
---@return pathobj

--- Extension.
function pathobj:extension(level) end
---@return string[]

--- Directory (return a new instance).
function pathobj:directory() end
---@return string[]

--- Absolute path (return a new instance).
function pathobj:absolute(rootdir) end

--- Relative path (return a new instance).
function pathobj:relative(rootdir) end

--- Join subpaths (return a new instance).
function pathobj:join(...) end

--- Split by separators.
function pathobj:split() end

--- Split PATH-like environment variable.
function pathobj:splitenv() end

