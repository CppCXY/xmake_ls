---@meta
---[configuration_option](https://xmake.io/#/manual/configuration_option)

---
---Defining options
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=option)
---
---@param name string Option name
---@return nil
function option(name) end

---
---End definition option
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=option_end)
---
---@return nil
function option_end() end

---
---**Scoped: option**
---
---Adding options depends
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionadd_deps)
---
---@param name string Depending option name
---@param ... string Depending option names
---@return nil
function add_deps(name, ...) end

---
---**Scoped: option**
---
---Adding options defines
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionadd_defines)
---
---@param name string Macro define name
---@param ... string Macro define names
---@return nil
function add_defines(name, ...) end

---
---**Scoped: option**
---
---Execute this script before option detection
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionbefore_check)
---
---@param func fun(option: Option): nil Function to run before check
---@return nil
function before_check(func) end

---
---**Scoped: option**
---
---Custom Option Detection Script
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionon_check)
---
---@param func fun(option: Option): nil Function to run for custom check
---@return nil
function on_check(func) end

---
---**Scoped: option**
---
---Execute this script after option detection
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionafter_check)
---
---@param func fun(option: Option): nil Function to run after check
---@return nil
function after_check(func) end

---
---**Scoped: option**
---
---Setting the list of option values
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionset_values)
---
---@param value string Option value
---@param ... string Option values
---@return nil
function set_values(value, ...) end

---
---**Scoped: option**
---
---Setting options defaults
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionset_default)
---
---@param default boolean|string Default value
---@return nil
function set_default(default) end

---
---**Scoped: option**
---
---Set whether to enable menu display
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionset_showmenu)
---
---@param enabled boolean Whether to enable menu
---@return nil
function set_showmenu(enabled) end

---
---**Scoped: option**
---
---Setting option categories, only for menu display
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionset_category)
---
---@param name string Menu category name
---@return nil
function set_category(name) end

---
---**Scoped: option**
---
---Setting menu display description
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionset_description)
---
---@param description string Menu description
---@param ... string Multiple line descriptions
---@return nil
function set_description(description, ...) end

---
---**Scoped: option**
---
---Add Link Library Detection
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionadd_links)
---
---@param link string Link library name
---@param ... string Link library names
---@return nil
function add_links(link, ...) end

---
---**Scoped: option**
---
---Adding the search directory needed for link library detection
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionadd_linkdirs)
---
---@param dir string Search link library directory
---@param ... string Search link library directories
---@return nil
function add_linkdirs(dir, ...) end

---
---**Scoped: optiOn**
---
---Adding a load search directory for a dynamic library at runtime
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optiOnadd_rpathdirs)
---
---@param dir string Load search directory for dynamic library
---@param ... string Load search directories for dynamic library
---@return nil
function add_rpathdirs(dir, ...) end

---
---**Scoped: option**
---
---Add c header file detection
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionadd_cincludes)
---
---@param name string File name
---@param ... string File names
---@return nil
function add_cincludes(name, ...) end

---
---**Scoped: option**
---
---Add c++ header file detection
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionadd_cxxincludes)
---
---@param name string File name
---@param ... string File names
---@return nil
function add_cxxincludes(name, ...) end

---
---**Scoped: option**
---
---Add c type detection
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionadd_ctypes)
---
---@param type string C type
---@param ... string C types
---@return nil
function add_ctypes(type, ...) end

---
---**Scoped: option**
---
---Adding c++ type detection
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionadd_cxxtypes)
---
---@param type string C++ type
---@param ... string C++ types
---@return nil
function add_cxxtypes(type, ...) end

---
---**Scoped: option**
---
---TODO: add option type
---
---Add c code fragment detection
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionadd_csnippets)
---
---@param name string Snippet name
---@param snippet string C snippet
---@param option? any Option
---@return nil
function add_csnippets(name, snippet, option) end

---
---**Scoped: option**
---
---TODO: add option type
---
---Adding c++ code snippet detection
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionadd_cxxsnippets)
---
---@param name string Snippet name
---@param snippet string C snippet
---@param option? any Option
---@return nil
function add_cxxsnippets(name, snippet, option) end

---
---**Scoped: option**
---
---Add c library function detection
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionadd_cfuncs)
---
---@param name string C function name
---@param ... string C function names
---@return nil
function add_cfuncs(name, ...) end

---
---**Scoped: option**
---
---Add c++ library function detection
---
---[Open in browser](https://xmake.io/#/manual/configuration_option?id=optionadd_cxxfuncs)
---
---@param name string C++ function name
---@param ... string C++ function names
---@return nil
function add_cxxfuncs(name, ...) end
