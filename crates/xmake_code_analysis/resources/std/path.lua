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
-- the License.

---
--- xmake 提供的路径工具模块，支持跨平台路径拼接、规范化、转换与环境变量路径处理。
--- 参考实现：xmake/core/base/path.lua
---@class pathlib
---@overload fun(p:string, transform?:fun(p:string):string): pathobj
path = {}

---@class pathobj
local pathobj = {}

--====================
-- 模块级函数 (path.*)
--====================

--- 规范化路径（折叠 ./、..、重复分隔符，并按平台分隔符格式化）
---@param p string|pathobj
---@return string
function path.normalize(p) end

--- 转换路径（可选 normalize=true）
---@param p string|pathobj
---@param opt? { normalize?: boolean }
---@return string
function path.translate(p, opt) end

--- 获取目录部分
---@param p string|pathobj
---@param sep? string 分隔符提示（已规范化时可加速）
---@return string
function path.directory(p, sep) end

--- 计算绝对路径（相对 rootdir）
---@param p string|pathobj
---@param rootdir? string
---@return string
function path.absolute(p, rootdir) end

--- 计算相对路径（相对 rootdir）
---@param p string|pathobj
---@param rootdir? string
---@return string
function path.relative(p, rootdir) end

--- 判断是否为绝对路径
---@param p string|pathobj
---@return boolean
function path.is_absolute(p) end

--- 转换为 unix 风格分隔符（主要用于 Windows）
---@param p string|pathobj
---@return string
function path.unix(p) end

--- 转换为 Cygwin 风格路径（例如 C:\ -> /c/）
---@param p string|pathobj
---@return string
function path.cygwin(p) end

--- 转换为 Msys/Cygwin 可用路径（"c:\xx" -> "/c/xx"）
---@param p string|pathobj
---@return string
function path.cygwin_path(p) end

--- 获取文件名（不含目录）
---@param p string|pathobj
---@param sep? string 分隔符提示（已规范化时可加速）
---@return string
function path.filename(p, sep) end

--- 获取不带扩展名的基名
---@param p string|pathobj
---@return string
function path.basename(p) end

--- 获取扩展名（含点），level>1 时返回多级扩展拼接
---@param p string|pathobj
---@param level? integer
---@return string
function path.extension(p, level) end

--- 拼接路径（自动加入分隔符并转换）
---@param p string|pathobj
---@param ... string|pathobj
---@return string
function path.join(p, ...) end

--- 按分隔符拆分路径
---@param p string|pathobj
---@return string[]
function path.split(p) end

--- 获取平台路径分隔符（'/' 或 '\\'）
---@return string
function path.sep() end

--- 获取环境变量 PATH 分隔符（':' 或 ';'）
---@return string
function path.envsep() end

--- 拆分 PATH 环境变量，处理引号/flag 等特殊情况
---@param env_path string
---@return string[]
function path.splitenv(env_path) end

--- 拼接 PATH 环境变量，自动处理引号/flag 等情况
---@param paths string[]
---@param envsep? string
---@return string
function path.joinenv(paths, envsep) end

--- 最后一个字符是否为分隔符
---@param p string|pathobj
---@return boolean
function path.islastsep(p) end

--- 将通配符转换为 Lua 模式（支持 * 与 **）
---@param pattern string
---@return string
function path.pattern(pattern) end

--- 新建 path 实例（等价于调用 path(p, transform)）
---@param p string|pathobj
---@param transform? fun(p:string):string 生成显示字符串的转换器
---@return pathobj
function path.new(p, transform) end

--- 是否为 path 实例
---@param p any
---@return boolean
function path.instance_of(p) end

--====================
-- 实例方法 (pathobj:*)
--====================

--- 获取展示字符串（应用 transform 后）
---@return string
function pathobj:str() end

--- 获取原始字符串
---@return string
function pathobj:rawstr() end

--- 设置原始字符串
---@param p string|pathobj
---@return pathobj
function pathobj:set(p) end

--- 是否为空
---@return boolean
function pathobj:empty() end

--- 设置 transform 函数
---@param transform fun(p:string):string
---@return pathobj
function pathobj:transform_set(transform) end

--- 获取 transform 函数
---@return fun(p:string):string|nil
function pathobj:transform_get() end

--- 克隆实例
---@return pathobj
function pathobj:clone() end

--- 规范化（返回新实例）
---@return pathobj
function pathobj:normalize() end

--- 路径转换（返回新实例）
---@param opt? { normalize?: boolean }
---@return pathobj
function pathobj:translate(opt) end

--- 转为 unix 风格（返回新实例）
---@return pathobj
function pathobj:unix() end

--- 文件名
---@return string
function pathobj:filename() end

--- 基名
---@return string
function pathobj:basename() end

--- 扩展名
---@param level? integer
---@return string
function pathobj:extension(level) end

--- 目录（返回新实例）
---@return pathobj
function pathobj:directory() end

--- 绝对路径（返回新实例）
---@param rootdir? string
---@return pathobj
function pathobj:absolute(rootdir) end

--- 相对路径（返回新实例）
---@param rootdir? string
---@return pathobj
function pathobj:relative(rootdir) end

--- 连接子路径（返回新实例）
---@param ... string|pathobj
---@return pathobj
function pathobj:join(...) end

--- 按分隔符拆分
---@return string[]
function pathobj:split() end

--- 拆分 PATH 环境变量
---@return string[]
function pathobj:splitenv() end

