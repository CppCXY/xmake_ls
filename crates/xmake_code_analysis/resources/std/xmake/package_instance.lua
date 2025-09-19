---@meta
---[package_instance](https://xmake.io/api/scripts/package-instance)

---@class Package
local Package = {}

---
---Get the name of the package
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-name)
---
---@return string
function Package:name() end

---
---Get the values of the package by name
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-get)
---
---@param key string Key
---@return string
function Package:get(key) end

---
---Set the values of the package by name (If you just want to add values use [package:add](#packageadd))
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-set)
---
---@param key string Key
---@param value string Value
---@return nil
function Package:set(key, value) end

---
---Add to the values of the package by name
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-add)
---
---@param key string Key
---@param value string Value
---@return nil
function Package:add(key, value) end

---
---Get the license of the package (Same as `package:get("license")`)
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-license)
---
---@return string
function Package:license() end

---
---Get the description of the package (Same as `package:get("description")`)
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-description)
---
---@return string
function Package:description() end

---
---Get the platform of the package.
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-plat)
---
---@return Platform
function Package:plat() end

---
---Get the architecture of the package (e.g. x86, x64, x86_64)
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-arch)
---
---@return Architecture
function Package:arch() end

---
---Get the targeted OS of the package. Can have the same values as [package:plat](#packageplat)
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-targetos)
---
---@return Platform
function Package:targetos() end

---
---Get the targeted architecture of the package. Can have the same values as [package:arch](#packagearch)
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-targetarch)
---
---@return Architecture
function Package:targetarch() end

---
---Whether the current platform is one of the given platforms
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-is_plat)
---
---@param platform Platform Checked platform
---@param ... Platform Checked platforms
---@return boolean
function Package:is_plat(platform, ...) end

---
---Whether the current platform is one of the given platforms
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-is_arch)
---
---@param arch Architecture Checked architecture
---@param ... Architecture Checked architectures
---@return boolean
function Package:is_arch(arch, ...) end

---
---Whether the currently targeted OS is one of the given OS
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-is_targetos)
---
---@param platform Platform Checked platform
---@param ... Platform Checked platforms
---@return boolean
function Package:is_targetos(platform, ...) end

---
---Whether the currently targeted architecture is one of the given architectures
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-is_targetarch)
---
---@param arch Architecture Checked architecture
---@param ... Architecture Checked architectures
---@return boolean
function Package:is_targetarch(arch, ...) end

---
---Get the alias of the package
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-alias)
---
---@return string
function Package:alias() end

---
---Get the URLs of the package
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-urls)
---
---@return string[]
function Package:urls() end

---
---Get a dependency of the package by name. The name needs to be a dependency of the package.
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-dep)
---
---@param name string Depending package name
---@return Package
function Package:dep(name) end

---
---Get all dependencies of the package
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-deps)
---
---@return { [string]: Package }
function Package:deps() end

---
---Get the sha256 checksum of an URL alias
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-sourcehash)
---
---@return string
function Package:sourcehash() end

---
---Get the kind of the package. Can be any of:
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-kind)
---
---@return PackageKind
function Package:kind() end

---
---Whether the package is of kind binary
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-is_binary)
---
---@return boolean
function Package:is_binary() end

---
---Whether the package is of kind toolchain
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-is_toolchain)
---
---@return boolean
function Package:is_toolchain() end

---
---Whether the package is of kind library
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-is_library)
---
---@return boolean
function Package:is_library() end

---
---Whether the package is directly required by the user (e.g. xmake.lua)
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-is_toplevel)
---
---@return boolean
function Package:is_toplevel() end

---
---Whether the package is provided by a thirdparty package manager (e.g. brew, conan, vcpkg)
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-is_thirdparty)
---
---@return boolean
function Package:is_thirdparty() end

---
---Whether the package is build with debug mode (Same as `package:config("debug")`)
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-is_debug)
---
---@return boolean
function Package:is_debug() end

---
---Whether the package is supported by the current platform and architecture
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-is_supported)
---
---@return boolean
function Package:is_supported() end

---@deprecated
---Whether the the package gets built with debug mode (deprecated: use [`package:is_debug`](#packageis_debug) instead)
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-debug)
---
---@return boolean
function Package:debug() end

---
---Whether the package is getting cross-compiled
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-is_cross)
---
---@return boolean
function Package:is_cross() end

---
---Get the cache directory of the package
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-cachedir)
---
---@return string
function Package:cachedir() end

---
---Get the installation directory of the package. Can also be used to get a subdirectory. If the given directory tree does not exist it will be created.
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-installdir)
---
---@param ... string Directory
---@return string
function Package:installdir(...) end

---
---Get the directory where the xmake.lua of the package lies
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-scriptdir)
---
---@return string
function Package:scriptdir() end

---
---Get the exported environment variables of the package
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-envs)
---
---@return { [string]: string[] }
function Package:envs() end

---
---Get the given environment variable
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-getenv)
---
---@param name string Environment name
---@return string[]
function Package:getenv(name) end

---
---Set the given environment variable. Overwrites the variable
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-setenv)
---
---@param name string Environment name
---@param value string Environment value
---@param ... string Environment value
---@return nil
function Package:setenv(name, value, ...) end

---
---Add the given values to the environment variable
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-addenv)
---
---@param name string Environment name
---@param value string Environment value
---@param ... string Environment value
---@return nil
function Package:addenv(name, value, ...) end

---
---Get all version strings of the package. Returns a table containing all versions as strings
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-versions)
---
---@return string[]
function Package:versions() end

---
---Get the version of the package
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-version)
---
---@return Version
function Package:version() end

---
---Get the version of the package as string
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-version_str)
---
---@return string
function Package:version_str() end

---
---Get the given configuration value of the package
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-config)
---
---@param key string Config key
---@return any
function Package:config(key) end

---
---Set the given configuration value of the package
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-config_set)
---
---@param key string Config key
---@param value any Config value
---@return nil
function Package:config_set(key, value) end

---
---Get all configurations of the package
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-configs)
---
---@return PackageConfig
function Package:configs() end

---
---Get the build hash of the package
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-buildhash)
---
---@return string
function Package:buildhash() end

---
---Get all patches of the current version
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-patches)
---
---@return PackagePatch[]
function Package:patches() end

---
---Whether the package has the given C functions
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-has_cfuncs)
--- Example:
--- ```lua
--- assert(package:has_cfuncs("zlibVersion", { includes = "zlib.h" }))
--- ```
---
---@param name string Function name
---@param ... string Function names
---@return boolean
function Package:has_cfuncs(name, ...) end

---
---Whether the package has the given C functions
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-has_cfuncs)
---
---@param name string Function name
---@param option? PackageCheckOption Option
---@return boolean
function Package:has_cfuncs(name, option) end

---
---Whether the package has the given C++ functions
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-has_cxxfuncs)
--- Example:
--- ```lua
--- assert(package:has_cxxfuncs("std::make_unique", { includes = "memory" }))
--- ```
---
---@param name string Function name
---@param ... string Function names
---@return boolean
function Package:has_cxxfuncs(name, ...) end

---
---Whether the package has the given C++ functions
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-has_cxxfuncs)
---
---@param name string Function name
---@param option? PackageCheckOption Option
---@return boolean
function Package:has_cxxfuncs(name, option) end

---
---Whether the package has the given C types
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-has_ctypes)
--- Example:
--- ```lua
--- assert(package:has_ctypes("z_stream", { includes = "zlib.h" }))
--- ```
---
---@param type string Type name
---@param ... string Type names
---@return boolean
function Package:has_ctypes(type, ...) end

---
---Whether the package has the given C types
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-has_ctypes)
---
---@param type string Type name
---@param option? PackageCheckOption Option
---@return boolean
function Package:has_ctypes(type, option) end

---
---Whether the package has the given C++ types
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-has_cxxtypes)
--- Example:
--- ```lua
--- assert(package:has_cxxtypes("std::string", { includes = "string" }))
--- ```
---
---@param type string Type name
---@param ... string Type names
---@return boolean
function Package:has_cxxtypes(type, ...) end

---
---Whether the package has the given C++ types
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-has_cxxtypes)
---
---@param type string Type name
---@param option? PackageCheckOption Option
---@return boolean
function Package:has_cxxtypes(type, option) end

---
---Whether the package has the given C header files
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-has_cincludes)
--- Example:
--- ```lua
--- assert(package:has_cincludes("zlib.h"))
--- ```
---
---@param file string Include file name
---@param ... string Include file names
---@return boolean
function Package:has_cincludes(file, ...) end

---
---Whether the package has the given C++ header files
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-has_cxxincludes)
--- Example:
--- ```lua
--- assert(package:has_cxxincludes("vector"))
--- ```
---
---@param file string Include file name
---@param ... string Include file names
---@return boolean
function Package:has_cxxincludes(file, ...) end

---
---Whether the given C snippet can be compiled and linked
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-check_csnippets)
--- Example:
--- ```lua
--- assert(package:check_csnippets([[ int add(int a,int b){return a+b;} ]], { includes = "stdio.h" }))
--- ```
---
---@param snippet PackageCheckSnippet Snippet
---@param ... PackageCheckSnippet Snippets
---@return boolean
function Package:check_csnippets(snippet, ...) end

---
---Whether the given C snippet can be compiled and linked
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-check_csnippets)
---
---@param snippet PackageCheckSnippet Snippet
---@param option? PackageCheckOption Option
---@return boolean
function Package:check_csnippets(snippet, option) end

---
---Whether the given C++ snippet can be compiled and linked
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-check_cxxsnippets)
--- Example:
--- ```lua
--- assert(package:check_cxxsnippets([[ #include <memory>
--- int main(){ auto p = std::make_unique<int>(1); return *p; } ]]))
--- ```
---
---@param snippet PackageCheckSnippet Snippet
---@param ... PackageCheckSnippet Snippets
---@return boolean
function Package:check_cxxsnippets(snippet, ...) end

---
---Whether the given C++ snippet can be compiled and linked
---
---[Open in browser](https://xmake.io/api/scripts/package-instance#package-check_cxxsnippets)
---
---@param snippet PackageCheckSnippet Snippet
---@param option? PackageCheckOption Option
---@return boolean
function Package:check_cxxsnippets(snippet, option) end
