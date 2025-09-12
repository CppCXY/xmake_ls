---@meta
---[project_target](https://xmake.io/#/manual/project_target)

---
---Define a project target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=target)
---
---@param target string Target name
---@param func? fun(): nil Target scoped function
---@return nil
function target(target, func) end

---
---Define a project target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=target)
---
---@param target string Target name
---@param attr TargetAttr Target attributes
---@return nil
function target(target, attr) end

---
---End target definition
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=target_end)
---
---@return nil
function target_end() end

---
---**Scoped: target**
---
---Set target kind
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_kind)
---
---@param kind TargetKind Target kind
---@return nil
function set_kind(kind) end

---
---**Scoped: target**
---
---Strip target symbols
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_strip)
---
---@param mode StripMode Symbols to strip
---@return nil
function set_strip(mode) end

---
---**Scoped: target**
---
---Enable or disable target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_enabled)
---
---@param enabled boolean Whether to enable this target
---@return nil
function set_enabled(enabled) end

---
---**Scoped: target**
---
---Mark as default target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_default)
---
---@param is_default boolean Whether this target is the default
---@return nil
function set_default(is_default) end

---
---**Scoped: target**
---
---Set configuration options
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_options)
---
---@param name string Option name
---@param ... string Option names
---@return nil
function set_options(name, ...) end

---
---**Scoped: target**
---
---Set symbol info
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_symbols)
---
---@param info SymbolInfo Symbol info
---@param ... SymbolInfo Symbol infos
---@return nil
function set_symbols(info, ...) end

---
---**Scoped: target**
---
---Set the base name of target file
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_basename)
---
---@param name string Base name
---@return nil
function set_basename(name) end

---
---**Scoped: target**
---
---Set the full name of target file
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_filename)
---
---@param name string Full name
---@return nil
function set_filename(name) end

---
---**Scoped: target**
---
---Set the leading name of the target file
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_prefixname)
---
---@param name string Prefix name
---@return nil
function set_prefixname(name) end

---
---**Scoped: target**
---
---Set the postname of the target file
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_suffixname)
---
---@param name string Suffix name
---@return nil
function set_suffixname(name) end

---
---**Scoped: target**
---
---Set the extension of the target file
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_extension)
---
---@param ext string Extension
---@return nil
function set_extension(ext) end

---
---**Scoped: target**
---
---Set compilation warning level
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_warnings)
---
---@param level WarningLevel Warning level
---@param ... WarningLevel Warning levels
---@return nil
function set_warnings(level, ...) end

---
---**Scoped: target**
---
---Set competition optimization level
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_optimize)
---
---@param level OptimizationLevel Compilation optimization level
---@return nil
function set_optimize(level) end

---
---**Scoped: target**
---
---Set source code language standards
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_languages)
---
---@param language LanguageStandard Language standard
---@param ... LanguageStandard Language standards
---@return nil
function set_languages(language, ...) end

---
---**Scoped: target**
---
---Set float-point compilation mode
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_fpmodels)
---
---@param model FloatPointModel Float-point model
---@param ... FloatPointModel Float-point models
---@return nil
function set_fpmodels(model, ...) end

---
---**Scoped: target**
---
---Set output directories for target files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_targetdir)
---
---@param dir string Target file output directory
---@return nil
function set_targetdir(dir) end

---
---**Scoped: target**
---
---Set output directories for object files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_objectdir)
---
---@param dir string Object file output directory
---@return nil
function set_objectdir(dir) end

---
---**Scoped: target**
---
---Set output directories for dependent files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_dependir)
---
---@param dir string Dependent file output directory
---@return nil
function set_dependir(dir) end

---
---**Scoped: target**
---
---Add imports modules for the custom script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_imports)
---
---@param name string Module name
---@param ... string Module names
---@return nil
function add_imports(name, ...) end

---
---**Scoped: target**
---
---Add custom compilation rule to target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_rules)
---
---@param name string Rule name
---@param ... string Rule names
---@return nil
function add_rules(name, ...) end

---
---**Scoped: target**
---
---Run custom load target configuration script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targeton_load)
---
---@param func TargetHook Function to run when target is loaded
---@return nil
function on_load(func) end

---
---**Scoped: target**
---
---custom configuration script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targeton_config)
---
---@param func TargetHook Function to run for custom config
---@return nil
function on_config(func) end

---
---**Scoped: target**
---
---Run custom link target script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targeton_link)
---
---@param func TargetHook Function to run when link
---@return nil
function on_link(func) end

---
---**Scoped: target**
---
---Run custom build target script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targeton_build)
---
---@param func TargetHook Function to run for custom build
---@return nil
function on_build(func) end

---
---**Scoped: target**
---
---Run custom build target script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targeton_build)
---
---@param platform_arch string Target platform and architecture
---@param func TargetHook Function to run for custom build
---@return nil
function on_build(platform_arch, func) end

---
---**Scoped: target**
---
---Run custom build single file script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targeton_build_file)
---
---@param func TargetBuildFileHook Function to run for custom build for single file
---@return nil
function on_build_file(func) end

---
---**Scoped: target**
---
---Run custom build files script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targeton_build_files)
---
---@param func TargetBuildFilesHook Function to run for custom build for batch files
---@return nil
function on_build_files(func) end

---
---**Scoped: target**
---
---Run custom clean files script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targeton_clean)
---
---@param func TargetHook Function to run for custom clean
---@return nil
function on_clean(func) end

---
---**Scoped: target**
---
---Run custom package target script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targeton_package)
---
---@param func TargetHook Function to run for custom packaging process
---@return nil
function on_package(func) end

---
---**Scoped: target**
---
---Run custom install target file script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targeton_install)
---
---@param func TargetHook Function to run for custom install
---@return nil
function on_install(func) end

---
---**Scoped: target**
---
---Run custom uninstall target file script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targeton_uninstall)
---
---@param func TargetHook Function to run for custom uninstall
---@return nil
function on_uninstall(func) end

---
---**Scoped: target**
---
---Run custom run target script
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targeton_run)
---
---@param func TargetHook Function to run for custom run
---@return nil
function on_run(func) end

---
---**Scoped: target**
---
---Run custom script before linking target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetbefore_link)
---
---@param func TargetHook Function to run before linking target
---@return nil
function before_link(func) end

---
---**Scoped: target**
---
---Run custom script before building target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetbefore_build)
---
---@param func TargetHook Function to run before building target
---@return nil
function before_build(func) end

---
---**Scoped: target**
---
---Run custom script before building single file
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetbefore_build_file)
---
---@param func TargetBuildFileHook Function to run before building single file
---@return nil
function before_build_file(func) end

---
---**Scoped: target**
---
---Run custom script before building files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetbefore_build_files)
---
---@param func TargetBuildFilesHook Function to run before building batch files
---@return nil
function before_build_files(func) end

---
---**Scoped: target**
---
---Run custom script before cleaning target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetbefore_clean)
---
---@param func TargetHook Function to run before cleaning target
---@return nil
function before_clean(func) end

---
---**Scoped: target**
---
---Run custom script before packaging target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetbefore_package)
---
---@param func TargetHook Function to run before packaging target
---@return nil
function before_package(func) end

---
---**Scoped: target**
---
---Run custom script before installing target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetbefore_install)
---
---@param func TargetHook Function to run before installing target
---@return nil
function before_install(func) end

---
---**Scoped: target**
---
---Run custom script before uninstalling target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetbefore_uninstall)
---
---@param func TargetHook Function to run before uninstalling target
---@return nil
function before_uninstall(func) end

---
---**Scoped: target**
---
---Run custom script before running target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetbefore_run)
---
---@param func TargetHook Function to run before running target
---@return nil
function before_run(func) end

---
---**Scoped: target**
---
---Run custom script after linking target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetafter_link)
---
---@param func TargetHook Function to run after linking target
---@return nil
function after_link(func) end

---
---**Scoped: target**
---
---Run custom script after building target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetafter_build)
---
---@param func TargetHook Function to run after building target
---@return nil
function after_build(func) end

---
---**Scoped: target**
---
---Run custom script after building single file
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetafter_build_file)
---
---@param func TargetBuildFileHook Function to run after building single file
---@return nil
function after_build_file(func) end

---
---**Scoped: target**
---
---Run custom script after building files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetafter_build_files)
---
---@param func TargetBuildFilesHook Function to run after building batch files
---@return nil
function after_build_files(func) end

---
---**Scoped: target**
---
---Run custom script after cleaning target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetafter_clean)
---
---@param func TargetHook Function to run after cleaning target
---@return nil
function after_clean(func) end

---
---**Scoped: target**
---
---Run custom script after packaging target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetafter_package)
---
---@param func TargetHook Function to run after packaging target
---@return nil
function after_package(func) end

---
---**Scoped: target**
---
---Run custom script after installing target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetafter_install)
---
---@param func TargetHook Function to run after installing target
---@return nil
function after_install(func) end

---
---**Scoped: target**
---
---Run custom script after uninstalling target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetafter_uninstall)
---
---@param func TargetHook Function to run after uninstalling target
---@return nil
function after_uninstall(func) end

---
---**Scoped: target**
---
---Run custom script after running target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetafter_run)
---
---@param func TargetHook Function to run after running target
---@return nil
function after_run(func) end

---
---**Scoped: target**
---
---Set pre-compiled c header file
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_pcheader)
---
---@param name string Pre-complied header file name
---@return nil
function set_pcheader(name) end

---
---**Scoped: target**
---
---Set pre-compiled c++ header file
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_pcxxheader)
---
---@param name string Pre-complied c++ header file name
---@return nil
function set_pcxxheader(name) end

---
---**Scoped: target**
---
---Add target dependencies
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_deps)
---
---@param name string Dependency name
---@param ... string Dependency names
---@return nil
function add_deps(name, ...) end

---TOOD: add_deps(name1, name2, access)
---
---**Scoped: target**
---
---Add target dependencies
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_deps)
---
---@param name string Dependency name
---@param access AccessSpecifier Access specifier
---@return nil
function add_deps(name, access) end

---
---**Scoped: target**
---
---Add link libraries
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_links)
---
---@param link string Link library
---@param ... string Link libraries
---@return nil
function add_links(link, ...) end

---
---**Scoped: target**
---
---Add system link libraries
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_syslinks)
---
---@param link string Link system library
---@param ... string Link system libraries
---@return nil
function add_syslinks(link, ...) end

---
---**Scoped: target**
---
---Add source files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_files)
---
---@param file string File name
---@param ... string File names
---@return nil
function add_files(file, ...) end

--- TODO: add_files(file1, file2, option)
---
---**Scoped: target**
---
---Add source files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_files)
---
---@param file string File name
---@param option CompilationOption Compilation option
---@return nil
function add_files(file, option) end

---
---**Scoped: target**
---
---Remove source files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetremove_files)
---
---@param file string File name
---@param ... string File names
---@return nil
function remove_files(file, ...) end

---
---**Scoped: target**
---
---Remove the specified file from the preceding list of header files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetremove_headerfiles)
---
---@param file string File name
---@param ... string File names
---@return nil
function remove_headerfiles(file, ...) end

---
---**Scoped: target**
---
---Add link search directories
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_linkdirs)
---
---@param dir string Link search directory
---@param ... string Link search directories
---@return nil
function add_linkdirs(dir, ...) end

---
---**Scoped: target**
---
---Add load search directories for dynamic libraries
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_rpathdirs)
---
---@param dir string Load search directory
---@param ... string Load search directories
---@return nil
function add_rpathdirs(dir, ...) end

---
---**Scoped: target**
---
---Add include search directories
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_includedirs)
---
---@param dir string Include directory
---@param ... string Include directories
---@return nil
function add_includedirs(dir, ...) end

---
---**Scoped: target**
---
---Add include search directories
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_includedirs)
---
---@param dir string Include directory
---@param access AccessSpecifier Access specifier
---@return nil
function add_includedirs(dir, access) end

---
---**Scoped: target**
---
---Add system header file search directory
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_sysincludedirs)
---
---@param dir string System header directory
---@param ... string System header directories
---@return nil
function add_sysincludedirs(dir, ...) end

---
---**Scoped: target**
---
---Add macro definition
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_defines)
---
---@param def string Macro define
---@param ... string Macro defines
---@return nil
function add_defines(def, ...) end

---
---**Scoped: target**
---
---Add macro undefinition
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_undefines)
---
---@param undef string Macro undef
---@param ... string Macro undefs
---@return nil
function add_undefines(undef, ...) end

---
---**Scoped: target**
---
---Add c compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_cflags)
---
---@param flag string C flag
---@param ... string C flags
---@return nil
function add_cflags(flag, ...) end

---TODO: add_cflags(flag1, flag2, attr)
---
---**Scoped: target**
---
---Add c compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_cflags)
---
---@param flag string C flag
---@param attr CompilationFlagAttr
---@return nil
function add_cflags(flag, attr) end

---
---**Scoped: target**
---
---Add c/c++ compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_cxflags)
---
---@param flag string C/C++ flag
---@param ... string C/C++ flags
---@return nil
function add_cxflags(flag, ...) end

---TODO: add_cxflags(flag1, flag2, attr)
---
---**Scoped: target**
---
---Add c/c++ compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_cxflags)
---
---@param flag string C/C++ flag
---@param attr CompilationFlagAttr
---@return nil
function add_cxflags(flag, attr) end

---
---**Scoped: target**
---
---Add c++ compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_cxxflags)
---
---@param flag string C++ flag
---@param ... string C++ flags
---@return nil
function add_cxxflags(flag, ...) end

---
---**Scoped: target**
---
---Add objc compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_mflags)
---
---@param flag string objc flag
---@param ... string objc flags
---@return nil
function add_mflags(flag, ...) end

---
---**Scoped: target**
---
---Add objc/objc++ compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_mxflags)
---
---@param flag string objc/objc++ flag
---@param ... string objc/objc++ flags
---@return nil
function add_mxflags(flag, ...) end

---
---**Scoped: target**
---
---Add objc++ compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_mxxflags)
---
---@param flag string objc++ flag
---@param ... string objc++ flags
---@return nil
function add_mxxflags(flag, ...) end

---
---**Scoped: target**
---
---Add swift compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_scflags)
---
---@param flag string swift flag
---@param ... string swift flags
---@return nil
function add_scflags(flag, ...) end

---
---**Scoped: target**
---
---Add asm compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_asflags)
---
---@param flag string asm flag
---@param ... string asm flags
---@return nil
function add_asflags(flag, ...) end

---
---**Scoped: target**
---
---Add go compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_gcflags)
---
---@param flag string go flag
---@param ... string go flags
---@return nil
function add_gcflags(flag, ...) end

---
---**Scoped: target**
---
---Add dlang compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_dcflags)
---
---@param flag string dlang flag
---@param ... string dlang flags
---@return nil
function add_dcflags(flag, ...) end

---
---**Scoped: target**
---
---Add rust compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_rcflags)
---
---@param flag string rust flag
---@param ... string rust flags
---@return nil
function add_rcflags(flag, ...) end

---
---**Scoped: target**
---
---Add fortran compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_fcflags)
---
---@param flag string fortran flag
---@param ... string fortran flags
---@return nil
function add_fcflags(flag, ...) end

---
---**Scoped: target**
---
---Add zig compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_zcflags)
---
---@param flag string zig flag
---@param ... string zig flags
---@return nil
function add_zcflags(flag, ...) end

---
---**Scoped: target**
---
---Add cuda compilation flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_cuflags)
---
---@param flag string cuda flag
---@param ... string cuda flags
---@return  nil
function add_cuflags(flag, ...) end

---
---**Scoped: target**
---
---Add cuda device link flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_culdflags)
---
---@param flag string cuda device link flag
---@param ... string cuda device link flag
---@return nil
function add_culdflags(flag, ...) end

---
---**Scoped: target**
---
---Add gencode settings for cuda devices
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_cugencodes)
---
---@param setting string gencode setting
---@param ... string gencode settings
---@return nil
function add_cugencodes(setting, ...) end

---
---**Scoped: target**
---
---Add static library link flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_ldflags)
---
---@param flag string Static library link flag
---@param ... string Static library link flags
---@return nil
function add_ldflags(flag, ...) end

---
---**Scoped: target**
---
---Add archive library flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_arflags)
---
---@param flag string Archive library flag
---@param ... string Archive library flags
---@return nil
function add_arflags(flag, ...) end

---
---**Scoped: target**
---
---Add dynamic library link flags
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_shflags)
---
---@param flag string Dynamic library link flag
---@param ... string Dynamic library link flags
---@return nil
function add_shflags(flag, ...) end

---
---**Scoped: target**
---
---Add option dependencies
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_options)
---
---@param name string Option name
---@param ... string Option names
---@return nil
function add_options(name, ...) end

---
---**Scoped: target**
---
---Add package dependencies
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_packages)
---
---@param name string Package name
---@param ... string Package names
---@return nil
function add_packages(name, ...) end

---
---**Scoped: target**
---
---Add package dependencies
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_packages)
---
---@param name string Package name
---@param attr { links: string|{} } Package attr
---@return nil
function add_packages(name, attr) end

---
---**Scoped: target**
---
---Add language standards
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_languages)
---
---@param language LanguageStandard Supported language
---@param ... LanguageStandard Supported languages
---@return nil
function add_languages(language, ...) end

---
---**Scoped: target**
---
---Add vector extensions
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_vectorexts)
---
---@param ext string Extension name
---@param ... string Extension names
---@return nil
function add_vectorexts(ext, ...) end

---
---**Scoped: target**
---
---Add frameworks
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_frameworks)
---
---@param framework string Framework name
---@param ... string Framework names
---@return nil
function add_frameworks(framework, ...) end

---
---**Scoped: target**
---
---Add framework search directories
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_frameworkdirs)
---
---@param dir string Framework directory
---@param ... string Framework directories
---@return nil
function add_frameworkdirs(dir, ...) end

---
---**Scoped: target**
---
---Set toolset
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_toolset)
---
---@param type ToolType Tool type
---@param name string Tool name or location
---@return nil
function set_toolset(type, name) end

---
---**Scoped: target**
---
---Set up the toolchain
---
---Run `xmake show -l toolchains` for full list
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_toolchains)
---
---@param name Toolchain Toolchain name
---@param ... Toolchain Toolchain names
---@return nil
function set_toolchains(name, ...) end

---
---**Scoped: target**
---
---Set up the toolchain
---Run `xmake show -l toolchains` for full list
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_toolchains)
---
---@param name Toolchain Toolchain name
---@param option ToolchainOption Toolchain option
---@return nil
function set_toolchains(name, option) end

---
---**Scoped: target**
---
---Set the compilation platform for the specified target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_plat)
---
---@param platform Platform Compilation platform
---@return nil
function set_plat(platform) end

---
---**Scoped: target**
---
---Set the compilation architecture of the specified target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_arch)
---
---@param arch Architecture Compilation architecture
---@return nil
function set_arch(arch) end

---
---**Scoped: target**
---
---Set custom configuration values
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_values)
---
---@param key string Custom config key
---@param value string Custom config value
---@param ... string Custom config values
---@return nil
function set_values(key, value, ...) end

---
---**Scoped: target**
---
---Add custom configuration values
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_values)
---
---@param key string Custom config key
---@param value string Custom config value
---@param ... string Custom config values
---@return nil
function add_values(key, value, ...) end

---
---**Scoped: target**
---
---Set the running directory
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_rundir)
---
---@param dir string Running directory
---@return nil
function set_rundir(dir) end

---
---**Scoped: target**
---
---Set the list of run parameters
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_runargs)
---
---@param arg string Running argument
---@param ... string Running arguments
---@return nil
function set_runargs(arg, ...) end

---
---**Scoped: target**
---
---Add runtime environment variables
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_runenvs)
---
---@param key string Environment key
---@param value string Environment value
---@param ... string Environment values
---@return nil
function add_runenvs(key, value, ...) end

---
---**Scoped: target**
---
---Set the runtime environment variable
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_runenv)
---
---@param key string Environment key
---@param value string Environment value
---@param ... string Environment values
---@return nil
function set_runenv(key, value, ...) end

---
---**Scoped: target**
---
---Set the installation directory
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_installdir)
---
---@param dir string Installation directory
---@return nil
function set_installdir(dir) end

---
---**Scoped: target**
---
---Add installation files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_installfiles)
---
---@param file string File name
---@param ... string File names
---@return nil
function add_installfiles(file, ...) end

---
---**Scoped: target**
---
---Add installation files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_installfiles)
---
---@param file string File name
---@param option InstallFilesOption Install files option
---@return nil
function add_installfiles(file, option) end

---
---**Scoped: target**
---
---Add header files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_headerfiles)
---
---@param file string File name
---@param ... string File names
---@return nil
function add_headerfiles(file, ...) end

---
---**Scoped: target**
---
---Add header files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_headerfiles)
---
---@param file string File name
---@param option InstallFilesOption Install files option
---@return nil
function add_headerfiles(file, option) end

---
---**Scoped: target**
---
---Set the output directory of configuration files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_configdir)
---
---@param dir string Output directory
---@return nil
function set_configdir(dir) end

---
---**Scoped: target**
---
---Set template configuration variables
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_configvar)
---
---@param key string Template config key
---@param value string Template config value
---@param option? ConfigvarOption Configvar option
---@return nil
function set_configvar(key, value, option) end

---
---**Scoped: target**
---
---Add template configuration files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_configfiles)
---
---@param file string Template config file name
---@param ... string Template config file names
---@return nil
function add_configfiles(file, ...) end

---
---**Scoped: target**
---
---Add template configuration files
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_configfiles)
---
---@param file string Template config file name
---@param option ConfigfilesOption Configfiles option
---@return nil
function add_configfiles(file, option) end

---
---**Scoped: target**
---
---Set build policy
---
---Run `xmake l core.project.policy.policies` for full list
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_policy)
---
---@param name BuildPolicy
---@param value any Policy value
---@param ... any Policy values
---@return nil
function set_policy(name, value, ...) end

---
---**Scoped: target**
---
---Set the runtime library of the compilation target
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_runtimes)
---
---@param runtime Runtime Runtime name
---@param ... Runtime Runtime names
---@return nil
function set_runtimes(runtime, ...) end

---
---**Scoped: target**
---
---Set target group
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_group)
---
---@param name string Group name
---@return nil
function set_group(name) end

---
---**Scoped: target**
---
---Add Source file groups
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetadd_filegroups)
---
---@param group string Path for the group
---@param option FilegroupsOption Filegroups option
---@return nil
function add_filegroups(group, option) end

---
---**Scoped: target**
---
---Enabling or disabling exceptions
---
---[Open in browser](https://xmake.io/#/manual/project_target?id=targetset_exceptions)
---
---@param type ExceptionType Exception type
---@param ... ExceptionType Exception types
---@return nil
function set_exceptions(type, ...) end
