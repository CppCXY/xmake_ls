---@meta
---xmake version: 2.7.7

---Supported operation system
---@alias OperationSystem string|"windows"|"linux"|"android"|"macosx"|"ios"

---Supported architecture
---@alias Architecture string

---Supported subarchitecture
---@alias Subarchitecture string

---Supported platform
---@alias Platform string|"windows"|"linux"|"macosx"|"android"|"iphoneos"|"watchos"

---Supported host
---@alias Host string|"windows"|"linux"|"macosx"

---Supported subhost
---@alias Subhost string|"msys"|"cygwin"

---Supported compilation mode
---@alias CompilationMode string|"debug"|"release"|"profile"

---Supported target kind
---@alias TargetKind string|"phony"|"binary"|"static"|"shared"|"object"|"headeronly"

---Supported runtime
---@alias Runtime string|"MT"|"MD"|"MTd"|"MDd"

---Supported strip mode
---@alias StripMode string|"debug"|"all"

---Supported symbol info
---@alias SymbolInfo string|"debug"|"debug,edit"|"debug,embed"|"hidden"

---Supported warning level
---@alias WarningLevel string|"none"|"less"|"more"|"all"|"allextra"|"everything"|"error"

---Supported optimization level
---@alias OptimizationLevel string|"none"|"fast"|"faster"|"fastest"|"smallest"|"aggressive"

---Supported language standard
---@alias LanguageStandard string|"ansi"|"c89"|"gnu89"|"c99"|"gnu99"|"c11"|"c17"|"clatest"|"cxx98"|"gnuxx98"|"cxx11"|"gnuxx11"|"cxx11"|"gnuxx11"|"cxx14"|"gnuxx14"|"cxx1z"|"gnuxx1z"|"cxx17"|"gnuxx17"|"cxx20"|"gnuxx20"|"cxxlatest"|"gnuxxlatest"

---Supported float-point model
---@alias FloatPointModel string|"fast"|"strict"|"fast"|"except"|"precise"

---Supported access specifier
---@class AccessSpecifier
---@field public public boolean
---@field public private boolean
---@field public inteface boolean
---@field public inherit boolean

---Supported compilation option
---@class CompilationOption
---@field rule string
---@field force CompilationOption
---@field defines string
---@field languages LanguageStandard
---@field includedirs string
---@field cflags string

---Supported compilation flag attr
---@class CompilationFlagAttr
---@field force boolean
---@field tools string

---Supported tool type
---@alias ToolType string|"cc"|"cxx"|"mm"|"mxx"|"gc"|"as"|"sc"|"rc"|"dc"|"fc"|"sc"|"rust"|"strip"|"ld"|"sh"|"ar"|"dcld"|"dcsh"

---Supported toolchain
---Run `xmake show -l toolchains` for full list
---@alias Toolchain string

---Supported toolchain option
---@class ToolchainOption
---@field plat Platform
---@field arch Architecture
---@field vs string vs version

---Supported install files option
---@class InstallFilesOption
---@field prefixdir string
---@field install boolean

---Supported configvar option
---@class ConfigvarOption
---@field quote boolean Whether to keep the quotes
---@field escape boolean Whether to escape char

---Supported configfile option
---@class ConfigfilesOption
---@field filename string
---@field prefixdir string
---@field onlycopy boolean
---@field pattern string
---@field variables { [string]: any}

---Supported build policy
---Run `xmake l core.project.policy.policies` for full list
---@alias BuildPolicy string

---Supported filegroups option
---@class FilegroupsOption
---@field rootdir string
---@field files string[]
---@field mode string|"plain"|"tree"

---Supported exception type
---@alias ExceptionType string|"cxx"|"no-cxx"|"objc"|"no-objc"

---@class SourceBatch
---@field sourcekind string Type of this batch of source files
---@field rulename string Build rule name
---@field sourcefiles string[] Get the list of source files
---@field objecetfiles string[] Get the list of object files
---@field dependfiles string[] Get the list of dependent files

---@class PackageConfig
---@field shared boolean Whether to enable dynamic library
---@field pic boolean Whether to enable pic
---@field vs_runtime Runtime MSVC runtime
---@field [string] any Other package configs

---@class RequiresOption
---@field optional boolean Whether the package is optional
---@field system boolean Whether to disable same system package
---@field verify boolean Whether to verify this package
---@field debug boolean Whether to enable debug mode
---@field alias string Another name for this package
---@field configs PackageConfig Package configs

---@class RequireconfsOption
---@field override boolean Whether to override existing configs
---@field version string Package version
---@field configs PackageConfig Package configs

---@class RepositoriesOption
---@field rootdir string Set root directory of repo

---TODO: add more
---@class TargetAttr
---@field kind TargetKind Set target kind
---@field files string[] Set target files
---@field includedirs string Set target include directories

---@alias TargetHook fun(target: Target): nil

---Built-in build script
---@class TargetOpt
---@field origin fun(target: Target, file: string|SourceBatch, opt: TargetOpt): nil Run built-in build script

---TODO: fix signature
---Built-in batch commands
---@class BatchCommand
---@field show fun(message: string, ...: string): nil Print messages
---@field show_progress fun(progress: number, message: string, file: string): nil Print progress
---@field vrunv fun(cmd: string, args: string[]): nil Run command with run environments
---@field mkdir fun(dir: string): nil Create directory
---@field compile fun(source: string, object: string, option: table): nil Compile source files
---@field link fun(object: string, target: string, option: table): nil Link object files
---@field add_depfiles fun(file: string, ...: string): nil Add depending files
---@field add_depvalues fun(value: string, ...: string): nil Add depending values
---@field add_depmtime fun(...: any): nil
---@field add_depcache fun(...: any): nil

---@alias TargetBuildFileHook fun(target: Target, file: string, opt: TargetOpt): nil
---@alias TargetBuildFilesHook fun(target: Target, files: SourceBatch, opt: TargetOpt): nil

---@alias TargetBuildcmdFileHook fun(target: Target, cmds: BatchCommand, file: string, opt: TargetOpt): nil
---@alias TargetBuildcmdFilesHook fun(target: Target, cmds: BatchCommand, files: SourceBatch, opt: TargetOpt): nil

---@alias TaskMenuOption string[]

---@class TaskMenu
---@field usage string Menu usage
---@field description string Menu description
---@field options TaskMenuOption[] Menu options

---@alias PackageKind string|"binary"|"toolchain"|"library"|"template"|"headeronly"

---@class PackageUrlOption
---@field version fun(version: string): string Convert version
---@field alias string Alias of this package link
---@field http_headers string[] Http headers for this package link

---@class PackageOpt
---@field sourcedir string Source directory
---@field url string Package url
---@field url_alias string Package url alias

---@alias PackageHook fun(package: Package): nil
---@alias PackageOptHook fun(package: Package, opt: PackageOpt): nil

---@alias Component any

---@alias PackageComponentHook fun(package: Package, component: Component): nil

---@class PackageConfigDefinition
---@field description string Description
---@field default any Default value for this config
---@field values any[] Value options

---@class Version
local Version = {}
---@return string
function Version:major() end
---@return string
function Version:major() end
---@return string
function Version:major() end

---@class PackagePatch
---@field url string Url of the patch
---@field sha256 string sha256 of the patch

---@class PackageCheckOptionConfig
---@field defines string Macro defines
---@field languages LanguageStandard Language
---@field [string] any Others

---@class PackageCheckOption
---@field includes string Include file
---@field configs PackageCheckOptionConfig Config

---@class PackageCheckSnippet
---@field test string Checked snippet
