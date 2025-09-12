---@meta
---[target_instance](https://xmake.io/#/manual/target_instance)

---@class Target
local Target = {}

---
---Get the name of the target
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetname)
---
---@return string
function Target:name() end

---
---Get the values of the target by name
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetget)
---
---@param key string Key
---@return string
function Target:get(key) end

---
---Set the values of the target by name (If you just want to add values use [target:add](#targetadd))
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetset)
---
---@param key string Key
---@param value any Value
---@return nil
function Target:set(key, value) end

---
---Add to the values of the target by name
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetadd)
---
---@param key string Key
---@param value any Value
---@return nil
function Target:add(key, value) end

---
---Get the target program type
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetkind)
---
---@return TargetKind
function Target:kind() end

---
---Whether the current platform is one of the given platforms
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetis_plat)
---
---@param platform Platform Platform name
---@param ... Platform Platform names
---@return boolean
function Target:is_plat(platform, ...) end

---
---Is the current architecture one of the given architectures
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetis_arch)
---
---@param arch Architecture Architecture name
---@param ... Architecture Architecture names
---@return boolean
function Target:is_arch(arch, ...) end

---
---Get the target file path
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targettargetfile)
---
---@return string
function Target:targetfile() end

---
---Get the output directory of the target file
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targettargetdir)
---
---@return string
function Target:targetdir() end

---
---Get the base name of the target file
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetbasename)
---
---@return string
function Target:basename() end

---
---Get the target file name
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetfilename)
---
---@return string
function Target:filename() end

---
---Get the installation directory of the target file
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetinstalldir)
---
---@return string
function Target:installdir() end

---
---Get auto-generated catalog
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetautogendir)
---
---@return string
function Target:autogendir() end

---
---Get the object file path
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetobjectfile)
---
---@param source string Source file name
---@return string
function Target:objectfile(source) end

---
---Get all source files
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetsourcebatches)
---
---@return { [string]: SourceBatch }
function Target:sourcebatches() end

---
---Get a list of all object files
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetobjectfiles)
---
---@return string[]
function Target:objectfiles() end

---
---Get a list of all header files
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetheaderfiles)
---
---@return string[]
function Target:headerfiles() end

---
---Get the xmake.lua directory where the target definition is located
---
---[Open in browser](https://xmake.io/#/manual/target_instance?id=targetscriptdir)
---
---@return string
function Target:scriptdir() end
