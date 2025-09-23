---@meta
---[custom_rule](https://xmake.io/api/description/custom-rule)

---
---Defining rules
---
---[Open in browser](https://xmake.io/api/description/custom-rule#rule)
---@scope rule
---@param name string Rule name
---@return nil
function rule(name) end

---
---Adding rule dependencies
---
---[Open in browser](https://xmake.io/api/description/custom-rule#add_deps)
---@scope rule
---@param name string Depending rule name
---@param ... string Depending rule names
---@return nil
function add_deps(name, ...) end

---
---Adding rule dependencies
---
---[Open in browser](https://xmake.io/api/description/custom-rule#add_deps)
---@scope rule
---@param name string Depending rule name
---@param option { order: boolean } Whether to execute in order
---@return nil
function add_deps(name, option) end

---
---Add imported modules for all custom scripts
---
---[Open in browser](https://xmake.io/api/description/custom-rule#add_imports)
---@scope rule
---@param name string Module name
---@param ... string Module names
---@return nil
function add_imports(name, ...) end

---
---Setting the file extension type supported by the rule
---
---[Open in browser](https://xmake.io/api/description/custom-rule#set_extensions)
---@scope rule
---@param ext string Extension type
---@param ... string Extension types
---@return nil
function set_extensions(ext, ...) end

---
---Custom load script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#on_load)
---@scope rule
---@param func TargetHook Function to run for custom load
---@return nil
function on_load(func) end

---
---custom configuration script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#on_config)
---@scope rule
---@param func TargetHook Function to run for custom config
---@return nil
function on_config(func) end

---
---Custom link script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#on_link)
---@scope rule
---@param func TargetHook Function to run when link
---@return nil
function on_link(func) end

---
---Custom compilation script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#on_build)
---@scope rule
---@param func TargetHook Function to run for custom build
---@return nil
function on_build(func) end

---
---Custom cleanup script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#on_clean)
---@scope rule
---@param func TargetHook Function to run for custom clean
---@return nil
function on_clean(func) end

---
---Custom packaging script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#on_package)
---@scope rule
---@param func TargetHook Function to run for custom packaging process
---@return nil
function on_package(func) end

---
---Custom installation script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#on_install)
---@scope rule
---@param func TargetHook Function to run for custom install
---@return nil
function on_install(func) end

---
---Custom Uninstall Script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#on_uninstall)
---@scope rule
---@param func TargetHook Function to run for custom uninstall
---@return nil
function on_uninstall(func) end

---
---Customizing the build script to process one source file at a time
---
---[Open in browser](https://xmake.io/api/description/custom-rule#on_build_file)
---@scope rule
---@param func TargetBuildFileHook Function to run for custom build for single file
---@return nil
function on_build_file(func) end

---
---Custom batch compile script, process one source file at a time
---
---[Open in browser](https://xmake.io/api/description/custom-rule#on_buildcmd_file)
---@scope rule
---@param func TargetBuildcmdFileHook Function to run for custom build
---@return nil
function on_buildcmd_file(func) end

---
---Customizing the build script to process multiple source files at once
---
---[Open in browser](https://xmake.io/api/description/custom-rule#on_build_files)
---@scope rule
---@param func TargetBuildFilesHook Function to run for custom build for batch files
---@return nil
function on_build_files(func) end

---
---Customize batch compiling script, process multiple source files at once
---
---[Open in browser](https://xmake.io/api/description/custom-rule#on_buildcmd_files)
---@scope rule
---@param func TargetBuildcmdFilesHook Function to run for custom build
---@return nil
function on_buildcmd_files(func) end

---
---Custom pre-link script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#before_link)
---@scope rule
---@param func TargetHook Function to run before linking target
---@return nil
function before_link(func) end

---
---Custom pre-compilation script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#before_build)
---@scope rule
---@param func TargetHook Function to run before building target
---@return nil
function before_build(func) end

---
---Custom pre-cleanup script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#before_clean)
---@scope rule
---@param func TargetHook Function to run before cleaning target
---@return nil
function before_clean(func) end

---
---Custom the pre-package script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#before_package)
---@scope rule
---@param func TargetHook Function to run before packaging target
---@return nil
function before_package(func) end

---
---Custom pre-installation script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#before_install)
---@scope rule
---@param func TargetHook Function to run before installing target
---@return nil
function before_install(func) end

---
---Custom pre-uninstall script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#before_uninstall)
---@scope rule
---@param func TargetHook Function to run before uninstalling target
---@return nil
function before_uninstall(func) end

---
---Custom pre-compilation script to process one source file at a time
---
---[Open in browser](https://xmake.io/api/description/custom-rule#before_build_file)
---@scope rule
---@param func TargetBuildFileHook Function to run before building single file
---@return nil
function before_build_file(func) end

---
---Customize the pre-compilation batch script, process one source file at a time
---
---[Open in browser](https://xmake.io/api/description/custom-rule#before_buildcmd_file)
---@scope rule
---@param func TargetBuildcmdFileHook Function to run before building
---@return nil
function before_buildcmd_file(func) end

---
---Customize pre-compilation scripts to process multiple source files at once
---
---[Open in browser](https://xmake.io/api/description/custom-rule#before_build_files)
---@scope rule
---@param func TargetBuildFilesHook Function to run before building batch files
---@return nil
function before_build_files(func) end

---
---Customize the pre-compilation batch script to process multiple source files at once
---
---[Open in browser](https://xmake.io/api/description/custom-rule#before_buildcmd_files)
---@scope rule
---@param func TargetBuildcmdFilesHook Function to run before building
---@return nil
function before_buildcmd_files(func) end

---
---Custom post-linking script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#after_link)
---@scope rule
---@param func TargetHook Function to run after linking target
---@return nil
function after_link(func) end

---
---Custom post-compilation script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#after_build)
---@scope rule
---@param func TargetHook Function to run after building target
---@return nil
function after_build(func) end

---
---Custom post-cleaning script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#after_clean)
---@scope rule
---@param func TargetHook Function to run after cleaning target
---@return nil
function after_clean(func) end

---
---Custom post-packaging script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#after_package)
---@scope rule
---@param func TargetHook Function to run after packaging target
---@return nil
function after_package(func) end

---
---Custom post-installation script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#after_install)
---@scope rule
---@param func TargetHook Function to run after installing target
---@return nil
function after_install(func) end

---
---Custom post-uninstallation Script
---
---[Open in browser](https://xmake.io/api/description/custom-rule#after_uninstall)
---@scope rule
---@param func TargetHook Function to run after uninstalling target
---@return nil
function after_uninstall(func) end

---
---Custom post-compilation scripts to process one source file at a time
---
---[Open in browser](https://xmake.io/api/description/custom-rule#after_build_file)
---@scope rule
---@param func TargetBuildFileHook Function to run after building single file
---@return nil
function after_build_file(func) end

---
---Customize the compiled batch script, process one source file at a time
---
---[Open in browser](https://xmake.io/api/description/custom-rule#after_buildcmd_file)
---@scope rule
---@param func TargetBuildcmdFileHook Function to run after building
---@return nil
function after_buildcmd_file(func) end

---
---Customize the compiled script to process multiple source files at once
---
---[Open in browser](https://xmake.io/api/description/custom-rule#after_build_files)
---@scope rule
---@param func TargetBuildFilesHook Function to run after building batch files
---@return nil
function after_build_files(func) end

---
---Customize the compiled batch script to process multiple source files at once
---
---[Open in browser](https://xmake.io/api/description/custom-rule#after_buildcmd_files)
---@scope rule
---@param func TargetBuildcmdFilesHook Function to run after building
---@return nil
function after_buildcmd_files(func) end

---
---End definition rules
---
---[Open in browser](https://xmake.io/api/description/custom-rule#rule_end)
---
---@return nil
function rule_end() end
