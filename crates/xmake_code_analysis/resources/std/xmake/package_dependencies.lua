---@meta
---[package_dependencies](https://xmake.io/#/manual/package_dependencies)

---
---Define package configuration
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=package)
---
---@param name string Package name
---@return nil
function package(name) end

---
---**Scoped: package**
---
---Set package homepage
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageset_homepage)
---
---@param link string Homepage link
---@return nil
function set_homepage(link) end

---
---**Scoped: package**
---
---Set package description
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageset_description)
---
---@param description string Package description
---@return nil
function set_description(description) end

---
---**Scoped: package**
---
---Set package kind
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageset_kind)
---
---@param kind PackageKind Package kind
---@return nil
function set_kind(kind) end

---
---**Scoped: package**
---
---Set package urls
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageset_urls)
---
---@param url string Package url
---@param option? PackageUrlOption Option
---@return nil
function set_urls(url, option) end

---
---**Scoped: package**
---
---Add package urls
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageadd_urls)
---
---@param url string Package url
---@param option? PackageUrlOption Option
---@return nil
function add_urls(url, option) end

---
---**Scoped: package**
---
---Add package versions
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageadd_versions)
---
---@param name string Package version
---@param sha256 string sha256 of the package
---@return nil
function add_versions(name, sha256) end

---
---**Scoped: package**
---
---Add package patches
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageadd_patches)
---
---@param version string Target package version
---@param patch string Patch file link
---@param sha256 string sha256 of patch
---@return nil
function add_patches(version, patch, sha256) end

---
---**Scoped: package**
---
---Add package links
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageadd_links)
---
---@param link string Library link
---@param ... string Library links
---@return nil
function add_links(link, ...) end

---
---**Scoped: package**
---
---Add system library links
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageadd_syslinks)
---
---@param link string System library link
---@param ... string System library links
---@return nil
function add_syslinks(link, ...) end

---
---**Scoped: package**
---
---Add frameworks
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageadd_frameworks)
---
---@param framework string Framework name
---@param ... string Framework names
---@return nil
function add_frameworks(framework, ...) end

---
---**Scoped: package**
---
---Add link directories
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageadd_linkdirs)
---
---@param dir string Link directory
---@param ... string Link directories
---@return nil
function add_linkdirs(dir, ...) end

---
---**Scoped: package**
---
---Add include directories
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageadd_includedirs)
---
---@param dir string Include directory
---@param ... string Include directories
---@return nil
function add_includedirs(dir, ...) end

---
---**Scoped: package**
---
---Add definition
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageadd_defines)
---
---@param def string Macro defines
---@param ... string Macro defines
---@return nil
function add_defines(def, ...) end

---
---**Scoped: package**
---
---Add package configs
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageadd_configs)
---
---@param name string Config name
---@param def PackageConfigDefinition Definition
---@return nil
function add_configs(name, def) end

---
---**Scoped: package**
---
---Add external package sources
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageadd_extsources)
---
---@param source string External package source
---@param ... string External package sources
---@return nil
function add_extsources(source, ...) end

---
---**Scoped: package**
---
---Add package dependencies
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageadd_deps)
---
---@param name string Depending package name
---@param ... string Depending package names
---@return nil
function add_deps(name, ...) end

---
---**Scoped: packages**
---
---Add package components
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packagesadd_components)
---
---@param name string Component name
---@param ... string Component names
---@return nil
function add_components(name, ...) end

---
---**Scoped: package**
---
---Inherit package configuration
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageset_base)
---
---@param name string Base package name
---@return nil
function set_base(name) end

---
---**Scoped: package**
---
---Load package configuration
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageon_load)
---
---@param func PackageHook Function to run when loading
---@return nil
function on_load(func) end

---
---**Scoped: package**
---
---Fetch package libraries
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageon_fetch)
---
---@param os OperationSystem Target operation system
---@param func PackageOptHook Function to run when fetching
---@return nil
function on_fetch(os, func) end

---
---TODO: `on_install(env1, env2, func)`
---
---**Scoped: package**
---
---Install package
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageon_install)
---
---@param func PackageHook Function to run when installing
---@return nil
function on_install(func) end

---
---**Scoped: package**
---
---Custom download package
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageon_download)
---
---@param func PackageOptHook Function to run when downloading
---@return nil
function on_download(func) end

---
---**Scoped: package**
---
---Test package
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageon_test)
---
---@param func PackageHook Function to run when testing
---@return nil
function on_test(func) end

---
---**Scoped: package**
---
---Define package component
---
---[Open in browser](https://xmake.io/#/manual/package_dependencies?id=packageon_componment)
---
---@param name string Componen name
---@param func PackageComponentHook Function to run for component
---@return nil
function on_componment(name, func) end
