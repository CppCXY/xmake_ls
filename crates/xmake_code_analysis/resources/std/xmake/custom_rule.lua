---@meta
---[custom_rule](https://xmake.io/#/manual/custom_rule)

---
---Defining rules
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=rule)
---
---@param name string Rule name
---@return nil
function rule(name) end

---
---**Scoped: rule**
---
---Adding rule dependencies
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleadd_deps)
---
---@param name string Depending rule name
---@param ... string Depending rule names
---@return nil
function add_deps(name, ...) end

---
---**Scoped: rule**
---
---Adding rule dependencies
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleadd_deps)
---
---@param name string Depending rule name
---@param option { order: boolean } Whether to execute in order
---@return nil
function add_deps(name, option) end

---
---**Scoped: rule**
---
---Add imported modules for all custom scripts
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleadd_imports)
---
---@param name string Module name
---@param ... string Module names
---@return nil
function add_imports(name, ...) end

---
---**Scoped: rule**
---
---Setting the file extension type supported by the rule
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleset_extensions)
---
---@param ext string Extension type
---@param ... string Extension types
---@return nil
function set_extensions(ext, ...) end

---
---**Scoped: rule**
---
---Custom load script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleon_load)
---
---@param func TargetHook Function to run for custom load
---@return nil
function on_load(func) end

---
---**Scoped: rule**
---
---custom configuration script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleon_config)
---
---@param func TargetHook Function to run for custom config
---@return nil
function on_config(func) end

---
---**Scoped: rule**
---
---Custom link script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleon_link)
---
---@param func TargetHook Function to run when link
---@return nil
function on_link(func) end

---
---**Scoped: rule**
---
---Custom compilation script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleon_build)
---
---@param func TargetHook Function to run for custom build
---@return nil
function on_build(func) end

---
---**Scoped: rule**
---
---Custom cleanup script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleon_clean)
---
---@param func TargetHook Function to run for custom clean
---@return nil
function on_clean(func) end

---
---**Scoped: rule**
---
---Custom packaging script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleon_package)
---
---@param func TargetHook Function to run for custom packaging process
---@return nil
function on_package(func) end

---
---**Scoped: rule**
---
---Custom installation script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleon_install)
---
---@param func TargetHook Function to run for custom install
---@return nil
function on_install(func) end

---
---**Scoped: rule**
---
---Custom Uninstall Script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleon_uninstall)
---
---@param func TargetHook Function to run for custom uninstall
---@return nil
function on_uninstall(func) end

---
---**Scoped: rule**
---
---Customizing the build script to process one source file at a time
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleon_build_file)
---
---@param func TargetBuildFileHook Function to run for custom build for single file
---@return nil
function on_build_file(func) end

---
---**Scoped: rule**
---
---Custom batch compile script, process one source file at a time
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleon_buildcmd_file)
---
---@param func TargetBuildcmdFileHook Function to run for custom build
---@return nil
function on_buildcmd_file(func) end

---
---**Scoped: rule**
---
---Customizing the build script to process multiple source files at once
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleon_build_files)
---
---@param func TargetBuildFilesHook Function to run for custom build for batch files
---@return nil
function on_build_files(func) end

---
---**Scoped: rule**
---
---Customize batch compiling script, process multiple source files at once
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleon_buildcmd_files)
---
---@param func TargetBuildcmdFilesHook Function to run for custom build
---@return nil
function on_buildcmd_files(func) end

---
---**Scoped: rule**
---
---Custom pre-link script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=rulebefore_link)
---
---@param func TargetHook Function to run before linking target
---@return nil
function before_link(func) end

---
---**Scoped: rule**
---
---Custom pre-compilation script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=rulebefore_build)
---
---@param func TargetHook Function to run before building target
---@return nil
function before_build(func) end

---
---**Scoped: rule**
---
---Custom pre-cleanup script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=rulebefore_clean)
---
---@param func TargetHook Function to run before cleaning target
---@return nil
function before_clean(func) end

---
---**Scoped: rule**
---
---Custom the pre-package script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=rulebefore_package)
---
---@param func TargetHook Function to run before packaging target
---@return nil
function before_package(func) end

---
---**Scoped: rule**
---
---Custom pre-installation script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=rulebefore_install)
---
---@param func TargetHook Function to run before installing target
---@return nil
function before_install(func) end

---
---**Scoped: rule**
---
---Custom pre-uninstall script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=rulebefore_uninstall)
---
---@param func TargetHook Function to run before uninstalling target
---@return nil
function before_uninstall(func) end

---
---**Scoped: rule**
---
---Custom pre-compilation script to process one source file at a time
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=rulebefore_build_file)
---
---@param func TargetBuildFileHook Function to run before building single file
---@return nil
function before_build_file(func) end

---
---**Scoped: rule**
---
---Customize the pre-compilation batch script, process one source file at a time
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=rulebefore_buildcmd_file)
---
---@param func TargetBuildcmdFileHook Function to run before building
---@return nil
function before_buildcmd_file(func) end

---
---**Scoped: rule**
---
---Customize pre-compilation scripts to process multiple source files at once
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=rulebefore_build_files)
---
---@param func TargetBuildFilesHook Function to run before building batch files
---@return nil
function before_build_files(func) end

---
---**Scoped: rule**
---
---Customize the pre-compilation batch script to process multiple source files at once
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=rulebefore_buildcmd_files)
---
---@param func TargetBuildcmdFilesHook Function to run before building
---@return nil
function before_buildcmd_files(func) end

---
---**Scoped: rule**
---
---Custom post-linking script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleafter_link)
---
---@param func TargetHook Function to run after linking target
---@return nil
function after_link(func) end

---
---**Scoped: rule**
---
---Custom post-compilation script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleafter_build)
---
---@param func TargetHook Function to run after building target
---@return nil
function after_build(func) end

---
---**Scoped: rule**
---
---Custom post-cleaning script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleafter_clean)
---
---@param func TargetHook Function to run after cleaning target
---@return nil
function after_clean(func) end

---
---**Scoped: rule**
---
---Custom post-packaging script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleafter_package)
---
---@param func TargetHook Function to run after packaging target
---@return nil
function after_package(func) end

---
---**Scoped: rule**
---
---Custom post-installation script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleafter_install)
---
---@param func TargetHook Function to run after installing target
---@return nil
function after_install(func) end

---
---**Scoped: rule**
---
---Custom post-uninstallation Script
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleafter_uninstall)
---
---@param func TargetHook Function to run after uninstalling target
---@return nil
function after_uninstall(func) end

---
---**Scoped: rule**
---
---Custom post-compilation scripts to process one source file at a time
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleafter_build_file)
---
---@param func TargetBuildFileHook Function to run after building single file
---@return nil
function after_build_file(func) end

---
---**Scoped: rule**
---
---Customize the compiled batch script, process one source file at a time
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleafter_buildcmd_file)
---
---@param func TargetBuildcmdFileHook Function to run after building
---@return nil
function after_buildcmd_file(func) end

---
---**Scoped: rule**
---
---Customize the compiled script to process multiple source files at once
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleafter_build_files)
---
---@param func TargetBuildFilesHook Function to run after building batch files
---@return nil
function after_build_files(func) end

---
---**Scoped: rule**
---
---Customize the compiled batch script to process multiple source files at once
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=ruleafter_buildcmd_files)
---
---@param func TargetBuildcmdFilesHook Function to run after building
---@return nil
function after_buildcmd_files(func) end

---
---End definition rules
---
---[Open in browser](https://xmake.io/#/manual/custom_rule?id=rule_end)
---
---@return nil
function rule_end() end
