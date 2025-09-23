---@meta
---[project_target](https://xmake.io/api/description/project-target)


---[Open in browser](https://xmake.io/api/description/project-target#target)
---
---@param target string Target name
---@param func? fun(): nil Target scoped function
---@return nil
function target(target, func) end

---
---Define a project target
---
---[Open in browser](https://xmake.io/api/description/project-target#target)
---
---@param target string Target name
---@param attr TargetAttr Target attributes
---@return nil
function target(target, attr) end

---
---End target definition
---
---[Open in browser](https://xmake.io/api/description/project-target#target_end)
---
---@return nil
function target_end() end

---
---Set target kind
---
---[Open in browser](https://xmake.io/api/description/project-target#set_kind)
---@scope target
---@param kind TargetKind Target kind
---@return nil
function set_kind(kind) end

---
---Strip target symbols
---
---[Open in browser](https://xmake.io/api/description/project-target#set_strip)
---@scope target
---@param mode StripMode Symbols to strip
---@return nil
function set_strip(mode) end

---
---Enable or disable target
---
---[Open in browser](https://xmake.io/api/description/project-target#set_enabled)
---@scope target
---@param enabled boolean Whether to enable this target
---@return nil
function set_enabled(enabled) end

---
---Mark as default target
---
---[Open in browser](https://xmake.io/api/description/project-target#set_default)
---@scope target
---@param is_default boolean Whether this target is the default
---@return nil
function set_default(is_default) end

---Set the full name of target file
---
---[Open in browser](https://xmake.io/api/description/project-target#set_filename)
---@scope target
---@param name string Full name
---@return nil
function set_filename(name) end

---
---Set the leading name of the target file
---
---[Open in browser](https://xmake.io/api/description/project-target#set_prefixname)
---@scope target
---@param name string Prefix name
---@return nil
function set_prefixname(name) end

---
---Set the postname of the target file
---
---[Open in browser](https://xmake.io/api/description/project-target#set_suffixname)
---@scope target
---@param name string Suffix name
---@return nil
function set_suffixname(name) end

---
---Set the extension of the target file
---
---[Open in browser](https://xmake.io/api/description/project-target#set_extension)
---@scope target
---@param ext string Extension
---@return nil
function set_extension(ext) end

---
---Set compilation warning level
---
---[Open in browser](https://xmake.io/api/description/project-target#set_warnings)
--- ```lua
--- target("app")
---     set_warnings("allextra")
--- ```
---@scope target
---@param level WarningLevel Warning level
---@param ... WarningLevel Warning levels
---@return nil
function set_warnings(level, ...) end

---
---Set competition optimization level
---
---[Open in browser](https://xmake.io/api/description/project-target#set_optimize)
--- ```lua
--- target("app")
---     set_optimize("faster")
--- ```
---@scope target
---@param level OptimizationLevel Compilation optimization level
---@return nil
function set_optimize(level) end

---
---Set source code language standards
---
---[Open in browser](https://xmake.io/api/description/project-target#set_languages)
--- ```lua
--- target("app")
---     set_languages("c99", "c++20")
--- ```
---@scope target
---@param language LanguageStandard Language standard
---@param ... LanguageStandard Language standards
---@return nil
function set_languages(language, ...) end

---
---Set float-point compilation mode
---
---[Open in browser](https://xmake.io/api/description/project-target#set_fpmodels)
---@scope target
---@param model FloatPointModel Float-point model
---@param ... FloatPointModel Float-point models
---@return nil
function set_fpmodels(model, ...) end

---
---Set output directories for target files
---
---[Open in browser](https://xmake.io/api/description/project-target#set_targetdir)
--- ```lua
--- target("app")
---     set_targetdir("build/bin")
--- ```
---@scope target
---@param dir string Target file output directory
---@return nil
function set_targetdir(dir) end

---
---Set output directories for object files
---
---[Open in browser](https://xmake.io/api/description/project-target#set_objectdir)
--- ```lua
--- target("app")
---     set_objectdir("build/obj")
--- ```
---@scope target
---@param dir string Object file output directory
---@return nil
function set_objectdir(dir) end

---
---Set output directories for dependent files
---
---[Open in browser](https://xmake.io/api/description/project-target#set_dependir)
--- ```lua
--- target("app")
---     set_dependir("build/.deps")
--- ```
---@scope target
---@param dir string Dependent file output directory
---@return nil
function set_dependir(dir) end

---
---Add imports modules for the custom script
---
---[Open in browser](https://xmake.io/api/description/project-target#add_imports)
--- target("app")
---     add_imports("core.project.config")
---@scope target
---@param name string Module name
---@param ... string Module names
---@return nil
function add_imports(name, ...) end

---
---Add custom compilation rule to target
---
---[Open in browser](https://xmake.io/api/description/project-target#add_rules)
--- target("app")
---     add_rules("mode.debug", "mode.release")
---@scope target
---@param name string Rule name
---@param ... string Rule names
---@return nil
function add_rules(name, ...) end

---
---Run custom load target configuration script
---
---[Open in browser](https://xmake.io/api/description/project-target#on_load)
--- target("app")
---     on_load(function (t)
---         print("loading target:", t:name())
---     end)
---@scope target
---@param func TargetHook Function to run when target is loaded
---@return nil
function on_load(func) end

---
---custom configuration script
---
---[Open in browser](https://xmake.io/api/description/project-target#on_config)
---@scope target
---@param func TargetHook Function to run for custom config
---@return nil
function on_config(func) end

---
---Run custom link target script
---
---[Open in browser](https://xmake.io/api/description/project-target#on_link)
---@scope target
---@param func TargetHook Function to run when link
---@return nil
function on_link(func) end

---
---Run custom build target script
---
---[Open in browser](https://xmake.io/api/description/project-target#on_build)
---@scope target
---@param func TargetHook Function to run for custom build
---@return nil
function on_build(func) end

---
---Run custom build target script
---
---[Open in browser](https://xmake.io/api/description/project-target#on_build)
---@scope target
---@param platform_arch string Target platform and architecture
---@param func TargetHook Function to run for custom build
---@return nil
function on_build(platform_arch, func) end

---
---Run custom build single file script
---
---[Open in browser](https://xmake.io/api/description/project-target#on_build_file)
---@scope target
---@param func TargetBuildFileHook Function to run for custom build for single file
---@return nil
function on_build_file(func) end

---
---Run custom build files script
---
---[Open in browser](https://xmake.io/api/description/project-target#on_build_files)
---@scope target
---@param func TargetBuildFilesHook Function to run for custom build for batch files
---@return nil
function on_build_files(func) end

---
---Run custom clean files script
---
---[Open in browser](https://xmake.io/api/description/project-target#on_clean)
--- ```lua
--- target("app")
---     on_clean(function (t)
---         print("cleaning:", t:name())
---     end)
--- ```
---@scope target
---@param func TargetHook Function to run for custom clean
---@return nil
function on_clean(func) end

---
---Run custom package target script
---
---[Open in browser](https://xmake.io/api/description/project-target#on_package)
---@scope target
---@param func TargetHook Function to run for custom packaging process
---@return nil
function on_package(func) end

---
---Run custom install target file script
---
---[Open in browser](https://xmake.io/api/description/project-target#on_install)
---@scope target
---@param func TargetHook Function to run for custom install
---@return nil
function on_install(func) end

---
---Run custom uninstall target file script
---
---[Open in browser](https://xmake.io/api/description/project-target#on_uninstall)
---@scope target
---@param func TargetHook Function to run for custom uninstall
---@return nil
function on_uninstall(func) end

---
---Run custom run target script
---
---[Open in browser](https://xmake.io/api/description/project-target#on_run)
--- ```lua
--- target("app")
---     on_run(function (t)
---         os.runv(t:targetfile(), {"--help"})
---     end)
--- ```
---@scope target
---@param func TargetHook Function to run for custom run
---@return nil
function on_run(func) end

---
---Run custom script before linking target
---
---[Open in browser](https://xmake.io/api/description/project-target#before_link)
---@scope target
---@param func TargetHook Function to run before linking target
---@return nil
function before_link(func) end

---
---Run custom script before building target
---
---[Open in browser](https://xmake.io/api/description/project-target#before_build)
---@scope target
---@param func TargetHook Function to run before building target
---@return nil
function before_build(func) end

---
---Run custom script before building single file
---
---[Open in browser](https://xmake.io/api/description/project-target#before_build_file)
---@scope target
---@param func TargetBuildFileHook Function to run before building single file
---@return nil
function before_build_file(func) end

---
---Run custom script before building files
---
---[Open in browser](https://xmake.io/api/description/project-target#before_build_files)
---@scope target
---@param func TargetBuildFilesHook Function to run before building batch files
---@return nil
function before_build_files(func) end

---
---Run custom script before cleaning target
---
---[Open in browser](https://xmake.io/api/description/project-target#before_clean)
---@scope target
---@param func TargetHook Function to run before cleaning target
---@return nil
function before_clean(func) end

---
---Run custom script before packaging target
---
---[Open in browser](https://xmake.io/api/description/project-target#before_package)
---@scope target
---@param func TargetHook Function to run before packaging target
---@return nil
function before_package(func) end

---
---Run custom script before installing target
---
---[Open in browser](https://xmake.io/api/description/project-target#before_install)
---@scope target
---@param func TargetHook Function to run before installing target
---@return nil
function before_install(func) end

---
---Run custom script before uninstalling target
---
---[Open in browser](https://xmake.io/api/description/project-target#before_uninstall)
---@scope target
---@param func TargetHook Function to run before uninstalling target
---@return nil
function before_uninstall(func) end

---
---Run custom script before running target
---
---[Open in browser](https://xmake.io/api/description/project-target#before_run)
---@scope target
---@param func TargetHook Function to run before running target
---@return nil
function before_run(func) end

---
---Run custom script after linking target
---
---[Open in browser](https://xmake.io/api/description/project-target#after_link)
---@scope target
---@param func TargetHook Function to run after linking target
---@return nil
function after_link(func) end

---
---Run custom script after building target
---
---[Open in browser](https://xmake.io/api/description/project-target#after_build)
---@scope target
---@param func TargetHook Function to run after building target
---@return nil
function after_build(func) end

---
---Run custom script after building single file
---
---[Open in browser](https://xmake.io/api/description/project-target#after_build_file)
---@scope target
---@param func TargetBuildFileHook Function to run after building single file
---@return nil
function after_build_file(func) end

---
---Run custom script after building files
---
---[Open in browser](https://xmake.io/api/description/project-target#after_build_files)
---@scope target
---@param func TargetBuildFilesHook Function to run after building batch files
---@return nil
function after_build_files(func) end

---
---Run custom script after cleaning target
---
---[Open in browser](https://xmake.io/api/description/project-target#after_clean)
---@scope target
---@param func TargetHook Function to run after cleaning target
---@return nil
function after_clean(func) end

---
---Run custom script after packaging target
---
---[Open in browser](https://xmake.io/api/description/project-target#after_package)
---@scope target
---@param func TargetHook Function to run after packaging target
---@return nil
function after_package(func) end

---
---Run custom script after installing target
---
---[Open in browser](https://xmake.io/api/description/project-target#after_install)
---@scope target
---@param func TargetHook Function to run after installing target
---@return nil
function after_install(func) end

---
---Run custom script after uninstalling target
---
---[Open in browser](https://xmake.io/api/description/project-target#after_uninstall)
---@scope target
---@param func TargetHook Function to run after uninstalling target
---@return nil
function after_uninstall(func) end

---
---Run custom script after running target
---
---[Open in browser](https://xmake.io/api/description/project-target#after_run)
---@scope target
---@param func TargetHook Function to run after running target
---@return nil
function after_run(func) end

---
---Set pre-compiled c header file
---
---[Open in browser](https://xmake.io/api/description/project-target#set_pcheader)
--- target("app")
---     set_pcheader("src/pch.h")
---@scope target
---@param name string Pre-complied header file name
---@return nil
function set_pcheader(name) end

---
---Set pre-compiled c++ header file
---
---[Open in browser](https://xmake.io/api/description/project-target#set_pcxxheader)
--- target("app")
---     set_pcxxheader("src/pch.hxx")
---@scope target
---@param name string Pre-complied c++ header file name
---@return nil
function set_pcxxheader(name) end

---
---Add target dependencies
---
---[Open in browser](https://xmake.io/api/description/project-target#add_deps)
---@scope target
---@param name string Dependency name
---@param ... string Dependency names
---@return nil
function add_deps(name, ...) end

---TODO: add_deps(name1, name2, access)
---
---Add target dependencies
---
---[Open in browser](https://xmake.io/api/description/project-target#add_deps)
---@scope target
---@param name string Dependency name
---@param access AccessSpecifier Access specifier
---@return nil
function add_deps(name, access) end

---
---Add link libraries
---
---[Open in browser](https://xmake.io/api/description/project-target#add_links)
---@scope target
---@param link string Link library
---@param ... string Link libraries
---@return nil
function add_links(link, ...) end

---
---Add system link libraries
---
---[Open in browser](https://xmake.io/api/description/project-target#add_syslinks)
---```lua
--- target("app")
---     if is_plat("windows") then
---         add_syslinks("user32", "ws2_32")
---     elseif is_plat("linux") then
---         add_syslinks("pthread", "dl")
---     end
---```
---@scope target
---@param link string Link system library
---@param ... string Link system libraries
---@return nil
function add_syslinks(link, ...) end

---
---Add source files
---
---[Open in browser](https://xmake.io/api/description/project-target#add_files)
---@scope target
---@param file string File name
---@param ... string File names
---@return nil
function add_files(file, ...) end

--- TODO: add_files(file1, file2, option)
---
---Add source files
---
---[Open in browser](https://xmake.io/api/description/project-target#add_files)
---@scope target
---@param file string File name
---@param option CompilationOption Compilation option
---@return nil
function add_files(file, option) end

---
---Remove source files
---
---[Open in browser](https://xmake.io/api/description/project-target#remove_files)
---```lua
--- target("app")
---     remove_files("src/legacy/**.c")
---```
---@scope target
---@param file string File name
---@param ... string File names
---@return nil
function remove_files(file, ...) end

---
---Remove the specified file from the preceding list of header files
---
---[Open in browser](https://xmake.io/api/description/project-target#remove_headerfiles)
--- ```lua
--- target("app")
---     remove_headerfiles("include/obsolete/**.h")
--- ```
---@scope target
---@param file string File name
---@param ... string File names
---@return nil
function remove_headerfiles(file, ...) end

---
---Add link search directories
---
---[Open in browser](https://xmake.io/api/description/project-target#add_linkdirs)
--- ```lua
--- target("app")
---     add_linkdirs("thirdparty/lib")
--- ```
---@scope target
---@param dir string Link search directory
---@param ... string Link search directories
---@return nil
function add_linkdirs(dir, ...) end

---
---Add load search directories for dynamic libraries
---
---[Open in browser](https://xmake.io/api/description/project-target#add_rpathdirs)
--- ```lua
--- target("app")
---     if is_plat("linux") then
---         add_rpathdirs("$ORIGIN/../lib")
---     end
--- ```
---@scope target
---@param dir string Load search directory
---@param ... string Load search directories
---@return nil
function add_rpathdirs(dir, ...) end

---
---Add include search directories
---
---[Open in browser](https://xmake.io/api/description/project-target#add_includedirs)
---@scope target
---@param dir string Include directory
---@param ... string Include directories
---@return nil
function add_includedirs(dir, ...) end

---
---Add include search directories
---
---[Open in browser](https://xmake.io/api/description/project-target#add_includedirs)
---@scope target
---@param dir string Include directory
---@param access AccessSpecifier Access specifier
---@return nil
function add_includedirs(dir, access) end

---
---Add system header file search directory
---
---[Open in browser](https://xmake.io/api/description/project-target#add_sysincludedirs)
--- ```lua
--- target("app")
---     add_sysincludedirs("/usr/include")
--- ```
---@scope target
---@param dir string System header directory
---@param ... string System header directories
---@return nil
function add_sysincludedirs(dir, ...) end

---
---Add macro definition
---
---[Open in browser](https://xmake.io/api/description/project-target#add_defines)
--- ```lua
--- target("app")
---     add_defines("NDEBUG")
--- ```
---@scope target
---@param def string Macro define
---@param ... string Macro defines
---@return nil
function add_defines(def, ...) end

---
---Add macro undefinition
---
---[Open in browser](https://xmake.io/api/description/project-target#add_undefines)
---@scope target
---@param undef string Macro undef
---@param ... string Macro undefs
---@return nil
function add_undefines(undef, ...) end

---
---Add c compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_cflags)
--- target("app")
---     add_cflags("-Wall", "-Wextra")
---@scope target
---@param flag string C flag
---@param ... string C flags
---@return nil
function add_cflags(flag, ...) end

---TODO: add_cflags(flag1, flag2, attr)
---
---Add c compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_cflags)
---@scope target
---@param flag string C flag
---@param attr CompilationFlagAttr
---@return nil
function add_cflags(flag, attr) end

---
---Add c/c++ compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_cxflags)
--- target("app")
---     add_cxflags("-fPIC")
---@scope target
---@param flag string C/C++ flag
---@param ... string C/C++ flags
---@return nil
function add_cxflags(flag, ...) end

---TODO: add_cxflags(flag1, flag2, attr)
---
---Add c/c++ compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_cxflags)
---@scope target
---@param flag string C/C++ flag
---@param attr CompilationFlagAttr
---@return nil
function add_cxflags(flag, attr) end

---
---Add c++ compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_cxxflags)
--- ```lua
--- target("app")
---     add_cxxflags("-stdlib=libc++")
--- ```
---@scope target
---@param flag string C++ flag
---@param ... string C++ flags
---@return nil
function add_cxxflags(flag, ...) end

---
---Add objc compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_mflags)
---@scope target
---@param flag string objc flag
---@param ... string objc flags
---@return nil
function add_mflags(flag, ...) end

---
---Add objc/objc++ compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_mxflags)
---@scope target
---@param flag string objc/objc++ flag
---@param ... string objc/objc++ flags
---@return nil
function add_mxflags(flag, ...) end

---
---Add objc++ compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_mxxflags)
---@scope target
---@param flag string objc++ flag
---@param ... string objc++ flags
---@return nil
function add_mxxflags(flag, ...) end

---
---Add swift compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_scflags)
---@scope target
---@param flag string swift flag
---@param ... string swift flags
---@return nil
function add_scflags(flag, ...) end

---
---Add asm compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_asflags)
---@scope target
---@param flag string asm flag
---@param ... string asm flags
---@return nil
function add_asflags(flag, ...) end

---
---Add go compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_gcflags)
---@scope target
---@param flag string go flag
---@param ... string go flags
---@return nil
function add_gcflags(flag, ...) end

---
---Add dlang compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_dcflags)
---@scope target
---@param flag string dlang flag
---@param ... string dlang flags
---@return nil
function add_dcflags(flag, ...) end

---
---Add rust compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_rcflags)
---@scope target
---@param flag string rust flag
---@param ... string rust flags
---@return nil
function add_rcflags(flag, ...) end

---
---Add fortran compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_fcflags)
---@scope target
---@param flag string fortran flag
---@param ... string fortran flags
---@return nil
function add_fcflags(flag, ...) end

---
---Add zig compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_zcflags)
---@scope target
---@param flag string zig flag
---@param ... string zig flags
---@return nil
function add_zcflags(flag, ...) end

---
---Add cuda compilation flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_cuflags)
---@scope target
---@param flag string cuda flag
---@param ... string cuda flags
---@return  nil
function add_cuflags(flag, ...) end

---
---Add cuda device link flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_culdflags)
---@scope target
---@param flag string cuda device link flag
---@param ... string cuda device link flag
---@return nil
function add_culdflags(flag, ...) end

---
---Add gencode settings for cuda devices
---
---[Open in browser](https://xmake.io/api/description/project-target#add_cugencodes)
---@scope target
---@param setting string gencode setting
---@param ... string gencode settings
---@return nil
function add_cugencodes(setting, ...) end

---
---Add static library link flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_ldflags)
--- ```lua
--- target("app")
---     if is_plat("linux") then
---         add_ldflags("-Wl,-rpath,$ORIGIN/../lib")
---     elseif is_plat("windows") then
---         add_ldflags("/DEBUG")
---     end
--- ```
---@scope target
---@param flag string Static library link flag
---@param ... string Static library link flags
---@return nil
function add_ldflags(flag, ...) end

---
---Add archive library flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_arflags)
---@scope target
---@param flag string Archive library flag
---@param ... string Archive library flags
---@return nil
function add_arflags(flag, ...) end

---
---Add dynamic library link flags
---
---[Open in browser](https://xmake.io/api/description/project-target#add_shflags)
---@scope target
---@param flag string Dynamic library link flag
---@param ... string Dynamic library link flags
---@return nil
function add_shflags(flag, ...) end

---
---Add option dependencies
---
---[Open in browser](https://xmake.io/api/description/project-target#add_options)
--- ```lua
--- target("app")
---     add_options("simd", "with_ssl")
--- ```
---@scope target
---@param name string Option name
---@param ... string Option names
---@return nil
function add_options(name, ...) end

---
---Add package dependencies
---
---[Open in browser](https://xmake.io/api/description/project-target#add_packages)
--- ```lua
--- target("app")
---     add_packages("openssl", "zlib")
--- ```
---@scope target
---@param name string Package name
---@param ... string Package names
---@return nil
function add_packages(name, ...) end

---
---Add package dependencies
---
---[Open in browser](https://xmake.io/api/description/project-target#add_packages)
---@scope target
---@param name string Package name
---@param attr { links: string|{} } Package attr
---@return nil
function add_packages(name, attr) end

---
---Add language standards
---
---[Open in browser](https://xmake.io/api/description/project-target#add_languages)
---@scope target
---@param language LanguageStandard Supported language
---@param ... LanguageStandard Supported languages
---@return nil
function add_languages(language, ...) end

---
---Add vector extensions
---
---[Open in browser](https://xmake.io/api/description/project-target#add_vectorexts)
--- ```lua
--- target("app")
---     add_vectorexts("sse2", "avx2")
--- ```
---@scope target
---@param ext string Extension name
---@param ... string Extension names
---@return nil
function add_vectorexts(ext, ...) end

---
---Add frameworks
---
---[Open in browser](https://xmake.io/api/description/project-target#add_frameworks)
--- ```lua
--- target("app")
---     if is_plat("macosx") then
---         add_frameworks("Cocoa", "OpenGL")
---     end
--- ```
---@scope target
---@param framework string Framework name
---@param ... string Framework names
---@return nil
function add_frameworks(framework, ...) end

---
---Add framework search directories
---
---[Open in browser](https://xmake.io/api/description/project-target#add_frameworkdirs)
--- ```lua
--- target("app")
---     if is_plat("macosx") then
---         add_frameworkdirs("/Library/Frameworks")
---     end
--- ```
---@scope target
---@param dir string Framework directory
---@param ... string Framework directories
---@return nil
function add_frameworkdirs(dir, ...) end

---
---Set toolset
---
---[Open in browser](https://xmake.io/api/description/project-target#set_toolset)
--- ```lua
--- target("app")
---     set_toolset("cc", "clang")
--- ```
---@scope target
---@param type ToolType Tool type
---@param name string Tool name or location
---@return nil
function set_toolset(type, name) end

---
---Set up the toolchain
---
---Run `xmake show -l toolchains` for full list
---
---[Open in browser](https://xmake.io/api/description/project-target#set_toolchains)
--- ```lua
--- target("app")
---     set_toolchains("clang")
--- ```
---@scope target
---@param name Toolchain Toolchain name
---@param ... Toolchain Toolchain names
---@return nil
function set_toolchains(name, ...) end

---
---Set up the toolchain
---Run `xmake show -l toolchains` for full list
---
---[Open in browser](https://xmake.io/api/description/project-target#set_toolchains)
---@scope target
---@param name Toolchain Toolchain name
---@param option ToolchainOption Toolchain option
---@return nil
function set_toolchains(name, option) end

---
---Set the compilation platform for the specified target
---
---[Open in browser](https://xmake.io/api/description/project-target#set_plat)
--- ```lua
--- target("app")
---     set_plat("windows")
--- ```
---@scope target
---@param platform Platform Compilation platform
---@return nil
function set_plat(platform) end

---
---Set the compilation architecture of the specified target
---
---[Open in browser](https://xmake.io/api/description/project-target#set_arch)
--- ```lua
--- target("app")
---     set_arch("x64")
--- ```
---@scope target
---@param arch Architecture Compilation architecture
---@return nil
function set_arch(arch) end

---
---Set custom configuration values
---
---[Open in browser](https://xmake.io/api/description/project-target#set_values)
---@scope target
---@param key string Custom config key
---@param value string Custom config value
---@param ... string Custom config values
---@return nil
function set_values(key, value, ...) end

---
---Add custom configuration values
---
---[Open in browser](https://xmake.io/api/description/project-target#add_values)
---@scope target
---@param key string Custom config key
---@param value string Custom config value
---@param ... string Custom config values
---@return nil
function add_values(key, value, ...) end

---
---Set the running directory
---
---[Open in browser](https://xmake.io/api/description/project-target#set_rundir)
--- ```lua
--- target("app")
---     set_rundir("build/run")
--- ```
---@scope target
---@param dir string Running directory
---@return nil
function set_rundir(dir) end

---
---Set the list of run parameters
---
---[Open in browser](https://xmake.io/api/description/project-target#set_runargs)
--- ```lua
--- target("app")
---     set_runargs("--verbose", "--threads=4")
--- ```
---@scope target
---@param arg string Running argument
---@param ... string Running arguments
---@return nil
function set_runargs(arg, ...) end

---
---Add runtime environment variables
---
---[Open in browser](https://xmake.io/api/description/project-target#add_runenvs)
--- ```lua
--- target("app")
---     add_runenvs("APP_ENV", "dev")
--- ```
---@scope target
---@param key string Environment key
---@param value string Environment value
---@param ... string Environment values
---@return nil
function add_runenvs(key, value, ...) end

---
---Set the runtime environment variable
---
---[Open in browser](https://xmake.io/api/description/project-target#set_runenv)
--- ```lua
--- target("app")
---     set_runenv("PATH", "$(projectdir)/tools:$(env PATH)")
--- ```
---@scope target
---@param key string Environment key
---@param value string Environment value
---@param ... string Environment values
---@return nil
function set_runenv(key, value, ...) end

---
---Set the installation directory
---
---[Open in browser](https://xmake.io/api/description/project-target#set_installdir)
--- ```lua
--- target("app")
---     set_installdir("dist")
--- ```
---@scope target
---@param dir string Installation directory
---@return nil
function set_installdir(dir) end

---
---Add installation files
---
---[Open in browser](https://xmake.io/api/description/project-target#add_installfiles)
--- ```lua
--- target("app")
---     add_installfiles("res/**")
--- ```
---@scope target
---@param file string File name
---@param ... string File names
---@return nil
function add_installfiles(file, ...) end

---
---Add installation files
---
---[Open in browser](https://xmake.io/api/description/project-target#add_installfiles)
---@scope target
---@param file string File name
---@param option InstallFilesOption Install files option
---@return nil
function add_installfiles(file, option) end

---
---Add header files
---
---[Open in browser](https://xmake.io/api/description/project-target#add_headerfiles)
---@scope target
---@param file string File name
---@param ... string File names
---@return nil
function add_headerfiles(file, ...) end

---
---Add header files
---
---[Open in browser](https://xmake.io/api/description/project-target#add_headerfiles)
---@scope target
---@param file string File name
---@param option InstallFilesOption Install files option
---@return nil
function add_headerfiles(file, option) end

---
---Set the output directory of configuration files
---
---[Open in browser](https://xmake.io/api/description/project-target#set_configdir)
---@scope target
---@param dir string Output directory
---@return nil
function set_configdir(dir) end

---
---Set template configuration variables
---
---[Open in browser](https://xmake.io/api/description/project-target#set_configvar)
---@scope target
---@param key string Template config key
---@param value string Template config value
---@param option? ConfigvarOption Configvar option
---@return nil
function set_configvar(key, value, option) end

---
---Add template configuration files
---
---[Open in browser](https://xmake.io/api/description/project-target#add_configfiles)
---@scope target
---@param file string Template config file name
---@param ... string Template config file names
---@return nil
function add_configfiles(file, ...) end

---
---Add template configuration files
---
---[Open in browser](https://xmake.io/api/description/project-target#add_configfiles)
---@scope target
---@param file string Template config file name
---@param option ConfigfilesOption Configfiles option
---@return nil
function add_configfiles(file, option) end

---
---Set build policy
---
---Run `xmake l core.project.policy.policies` for full list
---
---[Open in browser](https://xmake.io/api/description/project-target#set_policy)
---@scope target
---@param name BuildPolicy
---@param value any Policy value
---@param ... any Policy values
---@return nil
function set_policy(name, value, ...) end

---
---Set the runtime library of the compilation target
---
---[Open in browser](https://xmake.io/api/description/project-target#set_runtimes)
---@scope target
---@param runtime Runtime Runtime name
---@param ... Runtime Runtime names
---@return nil
function set_runtimes(runtime, ...) end

---
---Set target group
---
---[Open in browser](https://xmake.io/api/description/project-target#set_group)
--- ```lua
--- target("libfoo")
---     set_group("libs")
--- ```
---@scope target
---@param name string Group name
---@return nil
function set_group(name) end

---
---Add Source file groups
---
---[Open in browser](https://xmake.io/api/description/project-target#add_filegroups)
---@scope target
---@param group string Path for the group
---@param option FilegroupsOption Filegroups option
---@return nil
function add_filegroups(group, option) end

---
---Enabling or disabling exceptions
---
---[Open in browser](https://xmake.io/api/description/project-target#set_exceptions)
---@scope target
---@param type ExceptionType Exception type
---@param ... ExceptionType Exception types
---@return nil
function set_exceptions(type, ...) end
