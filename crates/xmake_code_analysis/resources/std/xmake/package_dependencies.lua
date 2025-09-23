---@meta
---[package_dependencies](https://xmake.io/api/description/package-dependencies)

---
---Define package configuration
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#package)
--- Example:
--- ```lua
--- package("zlib")
--- ```
---
---@param name string Package name
---@return nil
function package(name) end

---Set package homepage
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#set_homepage)
--- Example:
--- ```lua
--- set_homepage("https://zlib.net/")
--- ```
---@scope package
---@param link string Homepage link
---@return nil
function set_homepage(link) end

---Set package description
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#set_description)
--- Example:
--- ```lua
--- set_description("A Massively Spiffy Yet Delicately Unobtrusive Compression Library")
--- ```
---@scope package
---@param description string Package description
---@return nil
function set_description(description) end

---Set package kind
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#set_kind)
--- Example:
--- ```lua
--- set_kind("library")
--- ```
---@scope package
---@param kind PackageKind Package kind
---@return nil
function set_kind(kind) end

---Set package urls
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#set_urls)
--- Example:
--- ```lua
--- set_urls("https://example.com/src/v$(version).tar.gz")
--- ```
---@scope package
---@param url string Package url
---@param option? PackageUrlOption Option
---@return nil
function set_urls(url, option) end

---Add package urls
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_urls)
--- Example:
--- ```lua
--- add_urls("https://mirror/foo-$(version).tar.gz")
--- ```
---@scope package
---@param url string Package url
---@param option? PackageUrlOption Option
---@return nil
function add_urls(url, option) end

---Add package versions
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_versions)
--- Example:
--- ```lua
--- add_versions("1.2.3", "8b1a9953c4611296a827abf8c47804d7f8f5...")
--- add_versions("2.0.0", "sha256:deadbeef...", "sha256:cafebabe...")
--- ```
---@scope package
---@param name string Package version
---@param sha256 string sha256 of the package
---@return nil
function add_versions(name, sha256) end

---Add package patches
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_patches)
--- Example:
--- ```lua
--- add_patches("1.2.3", "fix-build.patch", "sha256:abcd...")
--- -- or patch from url
--- add_patches("1.2.3", "https://example.com/patch.diff", "sha256:ef01...")
--- ```
---@scope package
---@param version string Target package version
---@param patch string Patch file link
---@param sha256 string sha256 of patch
---@return nil
function add_patches(version, patch, sha256) end

---Add package links
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_links)
--- Example:
--- ```lua
--- add_links("z")
--- ```
--- Platform tips:
--- ```lua
--- if is_plat("windows") then
---     add_links("zlib")
--- elseif is_plat("linux") then
---     add_links("z")
--- end
--- ```
---@scope package
---@param link string Library link
---@param ... string Library links
---@return nil
function add_links(link, ...) end

---Add system library links
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_syslinks)
--- Example:
--- ```lua
--- add_syslinks("pthread", "m")
--- ```
--- Platform tips:
--- ```lua
--- if is_plat("linux") then
---     add_syslinks("pthread")
--- elseif is_plat("macosx") then
---     -- usually not needed on macOS for pthread
--- end
--- ```
---@scope package
---@param link string System library link
---@param ... string System library links
---@return nil
function add_syslinks(link, ...) end

---Add frameworks
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_frameworks)
--- Example:
--- ```lua
--- add_frameworks("CoreFoundation")
--- ```
--- Platform tips:
--- ```lua
--- if is_plat("macosx", "iphoneos") then
---     add_frameworks("CoreFoundation", "Security")
--- end
--- ```
---@scope package
---@param framework string Framework name
---@param ... string Framework names
---@return nil
function add_frameworks(framework, ...) end

---Add link directories
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_linkdirs)
--- Example:
--- ```lua
--- add_linkdirs("$(projectdir)/lib")
--- ```
--- Platform tips:
--- ```lua
--- if is_plat("windows") then
---     add_linkdirs("$(projectdir)/lib/win")
--- elseif is_plat("linux") then
---     add_linkdirs("$(projectdir)/lib/linux")
--- end
--- ```
---@scope package
---@param dir string Link directory
---@param ... string Link directories
---@return nil
function add_linkdirs(dir, ...) end

---Add include directories
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_includedirs)
--- Example:
--- ```lua
--- add_includedirs("$(projectdir)/include")
--- ```
--- Platform tips:
--- ```lua
--- if is_plat("windows") then
---     add_includedirs("$(projectdir)/include/win")
--- elseif is_plat("linux") then
---     add_includedirs("$(projectdir)/include/linux")
--- end
--- ```
---@scope package
---@param dir string Include directory
---@param ... string Include directories
---@return nil
function add_includedirs(dir, ...) end

---Add definition
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_defines)
--- Example:
--- ```lua
--- add_defines("FOO=1", "BAR")
--- ```
---@scope package
---@param def string Macro defines
---@param ... string Macro defines
---@return nil
function add_defines(def, ...) end

---Add package configs
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_configs)
--- Example:
--- ```lua
--- add_configs("shared", { description = "Build shared library", default = false })
--- add_configs("pic",    { description = "Enable position independent code", default = true })
--- ```
---@scope package
---@param name string Config name
---@param def PackageConfigDefinition Definition
---@return nil
function add_configs(name, def) end

---Add external package sources
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_extsources)
--- Example:
--- ```lua
--- add_extsources("cmake::zlib", "pkgconfig::zlib")
--- ```
---@scope package
---@param source string External package source
---@param ... string External package sources
---@return nil
function add_extsources(source, ...) end

---Add package dependencies
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_deps)
--- Example:
--- ```lua
--- add_deps("zlib", "openssl")
--- -- with optional config
--- add_deps("libpng", { configs = { shared = true } })
--- ```
---@scope package
---@param name string Depending package name
---@param ... string Depending package names
---@return nil
function add_deps(name, ...) end

---Add package components
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#add_components)
--- Example:
--- ```lua
--- add_components("core", "extras")
--- ```
---@scope package
---@param name string Component name
---@param ... string Component names
---@return nil
function add_components(name, ...) end

---Inherit package configuration
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#set_base)
--- Example:
--- ```lua
--- set_base("zlib_base")
--- ```
---@scope package
---@param name string Base package name
---@return nil
function set_base(name) end

---Load package configuration
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#on_load)
--- Example:
--- ```lua
--- on_load(function (package)
---     package:add("links", "z")
--- end)
--- ```
---@scope package
---@param func PackageHook Function to run when loading
---@return nil
function on_load(func) end

---Fetch package libraries
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#on_fetch)
--- Example:
--- ```lua
--- on_fetch("windows", function (package, opt)
---     return { links = "z", linkdirs = package:installdir("lib") }
--- end)
--- ```
---@scope package
---@param os OperationSystem Target operation system
---@param func PackageOptHook Function to run when fetching
---@return nil
function on_fetch(os, func) end

---
---TODO: `on_install(env1, env2, func)`
---Install package
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#on_install)
--- Example:
--- ```lua
--- on_install(function (package)
---     -- custom build and install
--- end)
--- ```
---@scope package
---@overload fun(...: string, func: PackageHook): nil
---@param func PackageHook Function to run when installing
---@return nil
function on_install(func, ...) end
---Custom download package
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#on_download)
--- Example:
--- ```lua
--- on_download(function (package, opt)
---     -- custom fetch to cache dir
--- end)
--- ```
---@scope package
---@param func PackageOptHook Function to run when downloading
---@return nil
function on_download(func) end

---Test package
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#on_test)
--- Example:
--- ```lua
--- on_test(function (package)
---     assert(package:has_cfuncs("zlibVersion", { includes = "zlib.h" }))
--- end)
--- ```
---@scope package
---@param func PackageHook Function to run when testing
---@return nil
function on_test(func) end

---Define package component
---
---[Open in browser](https://xmake.io/api/description/package-dependencies#on_component)
--- Example:
--- ```lua
--- on_componment("core", function (component)
---     component:add("links", "z")
--- end)
--- ```
---@scope package
---@param name string Componen name
---@param func PackageComponentHook Function to run for component
---@return nil
function on_componment(name, func) end
