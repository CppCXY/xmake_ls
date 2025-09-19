---@meta
---[conditions](https://xmake.io/api/description/conditions)

---
---Is the current compilation target system
---
---[Open in browser](https://xmake.io/api/description/conditions#is_os)
--- Example:
--- ```lua
--- if is_os("ios") then add_files("src/*.m") end
--- ```
---
---@param os OperationSystem Checked operation system
---@return boolean
function is_os(os) end

---
---Is the current compilation architecture
---
---[Open in browser](https://xmake.io/api/description/conditions#is_arch)
--- Example:
--- ```lua
--- if is_arch("x86_64", "i386") then add_files("src/*.c") end
--- ```
---
---@param arch Architecture Checked architecture
---@param ... Architecture Checked architectures
---@return boolean
function is_arch(arch, ...) end

---
---Is the current compilation platform
---
---[Open in browser](https://xmake.io/api/description/conditions#is_plat)
--- Example:
--- ```lua
--- if is_plat("windows", "linux") then
---     add_defines("PLATFORM_OK")
--- end
--- ```
---
---@param platform Platform Checked platform
---@param ... Platform Checked platforms
---@return boolean
function is_plat(platform, ...) end

---
---Is the current compilation host system
---
---[Open in browser](https://xmake.io/api/description/conditions#is_host)
--- Example:
--- ```lua
--- if is_host("macosx") then
---     add_defines("HOST_MAC")
--- end
--- ```
---
---@param host Host Checked host
---@return boolean
function is_host(host) end

---
---Determine the subsystem environment of the current host
---
---[Open in browser](https://xmake.io/api/description/conditions#is_subhost)
--- Example:
--- ```lua
--- if is_subhost("msys", "cygwin") then
---     add_defines("HOST_SUBSYSTEM")
--- end
--- ```
---
---@param subhost Subhost Checked subhost
---@param ... Subhost Checked subhosts
---@return boolean
function is_subhost(subhost, ...) end

---
---Determine the architecture of the current host subsystem environment
---
---[Open in browser](https://xmake.io/api/description/conditions#is_subarch)
--- Example:
--- ```lua
--- if is_subarch("x86", "x64") then
---     add_defines("HOST_SUBARCH")
--- end
--- ```
---
---@param subarch Subarchitecture Checked subarch
---@param ... Subarchitecture Checked architectures of current host subsystem environment
---@return boolean
function is_subarch(subarch, ...) end

---
---Is the current compilation mode
---
---[Open in browser](https://xmake.io/api/description/conditions#is_mode)
--- Example:
--- ```lua
--- if is_mode("debug") then
---     add_defines("DEBUG")
--- end
--- ```
---
---@param mode CompilationMode Checked compilation mode
---@return boolean
function is_mode(mode) end

---
---Is the current target kind
---
---[Open in browser](https://xmake.io/api/description/conditions#is_kind)
--- Example:
--- ```lua
--- if is_kind("binary") then
---     add_defines("APP_BIN")
--- end
--- ```
---
---@param kind TargetKind Checked target kind
---@return boolean
function is_kind(kind) end

---
---Is the given config values?
---
---[Open in browser](https://xmake.io/api/description/conditions#is_config)
--- Example:
--- ```lua
--- if is_config("runtime", "MD", "MDd") then
---     add_defines("USE_MSVC_MD")
--- end
--- ```
---
---@param key string Checked config key
---@param value string Checked config value
---@param ... string Checked config values
---@return boolean
function is_config(key, value, ...) end

---
---Is the given configs enabled?
---
---[Open in browser](https://xmake.io/api/description/conditions#has_config)
--- Example:
--- ```lua
--- if has_config("unitybuild") then
---     add_defines("UNITY_BUILD")
--- end
--- ```
---
---@param key string Checked config key
---@param ... string Checked config keys
---@return boolean
function has_config(key, ...) end

---
---Is the given dependent package enabled?
---
---[Open in browser](https://xmake.io/api/description/conditions#has_package)
--- Example:
--- ```lua
--- if has_package("openssl") then
---     add_defines("HAS_OPENSSL")
--- end
--- ```
---
---@param package string Checked package name
---@return boolean
function has_package(package) end
