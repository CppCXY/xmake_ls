---@meta
---[conditions](https://xmake.io/#/manual/conditions)

---
---Is the current compilation target system
---
---[Open in browser](https://xmake.io/#/manual/conditions?id=is_os)
---
---@param os OperationSystem Checked operation system
---@return boolean
function is_os(os) end

---
---Is the current compilation architecture
---
---[Open in browser](https://xmake.io/#/manual/conditions?id=is_arch)
---
---@param arch Architecture Checked architecture
---@param ... Architecture Checked architectures
---@return boolean
function is_arch(arch, ...) end

---
---Is the current compilation platform
---
---[Open in browser](https://xmake.io/#/manual/conditions?id=is_plat)
---
---@param platform Platform Checked platform
---@param ... Platform Checked platforms
---@return boolean
function is_plat(platform, ...) end

---
---Is the current compilation host system
---
---[Open in browser](https://xmake.io/#/manual/conditions?id=is_host)
---
---@param host Host Checked host
---@return boolean
function is_host(host) end

---
---Determine the subsystem environment of the current host
---
---[Open in browser](https://xmake.io/#/manual/conditions?id=is_subhost)
---
---@param subhost Subhost Checked subhost
---@param ... Subhost Checked subhosts
---@return boolean
function is_subhost(subhost, ...) end

---
---Determine the architecture of the current host subsystem environment
---
---[Open in browser](https://xmake.io/#/manual/conditions?id=is_subarch)
---
---@param subarch Subarchitecture Checked subarch
---@param ... Subarchitecture Checked architectures of current host subsystem environment
---@return boolean
function is_subarch(subarch, ...) end

---
---Is the current compilation mode
---
---[Open in browser](https://xmake.io/#/manual/conditions?id=is_mode)
---
---@param mode CompilationMode Checked compilation mode
---@return boolean
function is_mode(mode) end

---
---Is the current target kind
---
---[Open in browser](https://xmake.io/#/manual/conditions?id=is_kind)
---
---@param kind TargetKind Checked target kind
---@return boolean
function is_kind(kind) end

---
---Is the given config values?
---
---[Open in browser](https://xmake.io/#/manual/conditions?id=is_config)
---
---@param key string Checked config key
---@param value string Checked config value
---@param ... string Checked config values
---@return boolean
function is_config(key, value, ...) end

---
---Is the given configs enabled?
---
---[Open in browser](https://xmake.io/#/manual/conditions?id=has_config)
---
---@pram key string Checked config key
---@param ... string Checked config keys
---@return boolean
function has_config(key, ...) end

---
---Is the given dependent package enabled?
---
---[Open in browser](https://xmake.io/#/manual/conditions?id=has_package)
---
---@param package string Checked package name
---@return boolean
function has_package(package) end
