---@meta
---[configuration_option](https://xmake.io/api/description/configuration-option)

---
---Defining options
---
---[Open in browser](https://xmake.io/api/description/configuration-option#option)
--- Example:
--- ```lua
--- option("enable_gui")
---     set_default(false)
---     set_showmenu(true)
--- ```
---@scope option
---@param name string Option name
---@return nil
function option(name) end

---
---End definition option
---
---[Open in browser](https://xmake.io/api/description/configuration-option#option-end)
---
---@return nil
function option_end() end

---
---Adding options depends
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_deps)
---@scope option
---@param name string Depending option name
---@param ... string Depending option names
---@return nil
function add_deps(name, ...) end

---
---Adding options defines
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_defines)
---@scope option
---@param name string Macro define name
---@param ... string Macro define names
---@return nil
function add_defines(name, ...) end

---
---Execute this script before option detection
---
---[Open in browser](https://xmake.io/api/description/configuration-option#before_check)
---@scope option
---@param func fun(option: Option): nil Function to run before check
---@return nil
function before_check(func) end

---
---Custom Option Detection Script
---
---[Open in browser](https://xmake.io/api/description/configuration-option#on_check)
---@scope option
---@param func fun(option: Option): nil Function to run for custom check
---@return nil
function on_check(func) end

---
---Execute this script after option detection
---
---[Open in browser](https://xmake.io/api/description/configuration-option#after_check)
---@scope option
---@param func fun(option: Option): nil Function to run after check
---@return nil
function after_check(func) end

---
---Setting the list of option values
---
---[Open in browser](https://xmake.io/api/description/configuration-option#set_values)
---@scope option
---@param value string Option value
---@param ... string Option values
---@return nil
function set_values(value, ...) end

---
---Setting options defaults
---
---[Open in browser](https://xmake.io/api/description/configuration-option#set_default)
---@scope option
---@param default boolean|string Default value
---@return nil
function set_default(default) end

---
---Set whether to enable menu display
---
---[Open in browser](https://xmake.io/api/description/configuration-option#set_showmenu)
---@scope option
---@param enabled boolean Whether to enable menu
---@return nil
function set_showmenu(enabled) end

---
---Setting option categories, only for menu display
---
---[Open in browser](https://xmake.io/api/description/configuration-option#set_category)
---@scope option
---@param name string Menu category name
---@return nil
function set_category(name) end

---
---Setting menu display description
---
---[Open in browser](https://xmake.io/api/description/configuration-option#set_description)
---@scope option
---@param description string Menu description
---@param ... string Multiple line descriptions
---@return nil
function set_description(description, ...) end

---
---Add Link Library Detection
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_links)
---@scope option
---@param link string Link library name
---@param ... string Link library names
---@return nil
function add_links(link, ...) end

---
---Adding the search directory needed for link library detection
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_linkdirs)
---@scope option
---@param dir string Search link library directory
---@param ... string Search link library directories
---@return nil
function add_linkdirs(dir, ...) end

---
---Adding a load search directory for a dynamic library at runtime
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_rpathdirs)
---@scope option
---@param dir string Load search directory for dynamic library
---@param ... string Load search directories for dynamic library
---@return nil
function add_rpathdirs(dir, ...) end

---
---Add c header file detection
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_cincludes)
---@scope option
---@param name string File name
---@param ... string File names
---@return nil
function add_cincludes(name, ...) end

---
---Add c++ header file detection
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_cxxincludes)
---@scope option
---@param name string File name
---@param ... string File names
---@return nil
function add_cxxincludes(name, ...) end

---
---Add c type detection
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_ctypes)
---@scope option
---@param type string C type
---@param ... string C types
---@return nil
function add_ctypes(type, ...) end

---
---Adding c++ type detection
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_cxxtypes)
---@scope option
---@param type string C++ type
---@param ... string C++ types
---@return nil
function add_cxxtypes(type, ...) end

---
---TODO: add option type
---
---Add c code fragment detection
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_csnippets)
---@scope option
---@param name string Snippet name
---@param snippet string C snippet
---@param option? any Option
---@return nil
function add_csnippets(name, snippet, option) end

---
---TODO: add option type
---
---Adding c++ code snippet detection
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_cxxsnippets)
--- Example:
--- ```lua
--- add_cxxsnippets("int add(int a,int b){return a+b;}")
--- ```
---@scope option
---@param name string Snippet name
---@param snippet string C snippet
---@param option? any Option
---@return nil
function add_cxxsnippets(name, snippet, option) end

---
---Add c library function detection
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_cfuncs)
---@scope option
---@param name string C function name
---@param ... string C function names
---@return nil
function add_cfuncs(name, ...) end

---
---Add c++ library function detection
---
---[Open in browser](https://xmake.io/api/description/configuration-option#add_cxxfuncs)
---@scope option
---@param name string C++ function name
---@param ... string C++ function names
---@return nil
function add_cxxfuncs(name, ...) end
