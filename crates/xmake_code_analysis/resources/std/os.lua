---@meta
-- Copyright (c) 2018. tangzx(love.tangzx@qq.com)
--
-- Licensed under the Apache License, Version 2.0 (the "License"); you may not
-- use this file except in compliance with the License. You may obtain a copy of
-- the License at
--
-- http://www.apache.org/licenses/LICENSE-2.0
--
-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
-- WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
-- License for the specific language governing permissions and limitations under
-- the License.

---@class oslib
os = {}

---
--- Returns an approximation of the amount in seconds of CPU time used by
--- the program.
---@return number
function os.clock() end

---@class std.osdate
---@field year integer|string? four digits
---@field month integer|string? 1-12
---@field day integer|string? 1-31
---@field hour integer|string? 0-23
---@field min integer|string? 0-59
---@field sec integer|string? 0-61, due to leap seconds
---@field wday integer|string? 1-7, Sunday is 1
---@field yday integer|string? 1-366
---@field isdst boolean? daylight saving flag, a boolean.

---
--- Returns a string or a table containing date and time, formatted according
--- to the given string `format`.
---
--- If the `time` argument is present, this is the time to be formatted (see
--- the `os.time` function for a description of this value). Otherwise,
--- `date` formats the current time.
---
--- If `format` starts with '`!`', then the date is formatted in Coordinated
--- Universal Time. After this optional character, if `format` is the string
--- "`*t`", then `date` returns a table with the following fields:
---
--- **`year`** (four digits)
--- **`month`** (1–12)
--- **`day`** (1-31)
--- **`hour`** (0-23)
--- **`min`** (0-59)
--- **`sec`** (0-61), due to leap seconds
--- **`wday`** (weekday, 1–7, Sunday is 1)
--- **`yday`** (day of the year, 1–366)
--- **`isdst`** (daylight saving flag, a boolean). This last field may be absent
--- if the information is not available.
---
--- If `format` is not "`*t`", then `date` returns the date as a string,
--- formatted according to the same rules as the ISO C function `strftime`.
---
--- When called without arguments, `date` returns a reasonable date and time
--- representation that depends on the host system and on the current locale.
--- (More specifically, `os.date()` is equivalent to `os.date("%c")`.)
---
--- On non-POSIX systems, this function may be not thread safe because of its
--- reliance on C function `gmtime` and C function `localtime`.
---@overload fun(fmt:"*t", time: number):table
---@overload fun(fmt:"!*t", time: number):table
---@param format string
---@param time? number
---@return string|std.osdate
function os.date(format, time) end

---
--- Returns the difference, in seconds, from time `t1` to time `t2`. (where the
--- times are values returned by `os.time`). In POSIX, Windows, and some other
--- systems, this value is exactly `t2`-`t1`.
---@param t2 number
---@param t1 number
---@return number
function os.difftime(t2, t1) end

--- @version > 5.2
---
--- This function is equivalent to the C function `system`. It passes `command`
--- to be executed by an operating system shell. Its first result is **true** if
--- the command terminated successfully, or **nil** otherwise. After this first
--- result the function returns a string plus a number, as follows:
---
--- **"exit"**: the command terminated normally; the following number is the
--- exit status of the command.
--- **"signal"**: the command was terminated by a signal; the following number
--- is the signal that terminated the command.
---
--- When called without a command, `os.execute` returns a boolean that is true
--- if a shell is available.
--- @overload fun():boolean
--- @param command string
--- @return true|nil
--- @return 'exit'|'signal'
--- @return integer
function os.execute(command) end

--- @version 5.1, JIT
---
--- This function is equivalent to the C function system. It passes command to
--- be executed by an operating system shell. It returns a status code, which is
--- system-dependent. If command is absent, then it returns nonzero if a shell
--- is available and zero otherwise.
--- @param command string
--- @return integer
function os.execute(command) end

--- @version > 5.2, JIT
---
--- Calls the ISO C function `exit` to terminate the host program. If `code` is
--- **true**, the returned status is `EXIT_SUCCESS`; if `code` is **false**, the
--- returned status is `EXIT_FAILURE`; if `code` is a number, the returned
--- status is this number. The default value for `code` is **true**.
---
--- If the optional second argument `close` is true, closes the Lua state before
--- exiting.
---@param code integer
---@param close? boolean
---@return integer
function os.exit(code, close) end

--- @version 5.1
---
--- Calls the C function exit, with an optional `code`, to terminate the host
--- program. The default value for `code` is the success code.
---@param code integer
---@return integer
function os.exit(code) end

---
--- Returns the value of the process environment variable `varname`, or
--- **nil** if the variable is not defined.
---@param varname string
---@return string?
function os.getenv(varname) end

---
--- Deletes the file (or empty directory, on POSIX systems) with the given name.
--- If this function fails, it returns **nil**, plus a string describing the
--- error and the error code. Otherwise, it returns true.
---@param filename string
---@return true|nil result
---@return string err
function os.remove(filename) end

---
--- Renames the file or directory named `oldname` to `newname`. If this function
--- fails, it returns **nil**, plus a string describing the error and the error
--- code. Otherwise, it returns true.
---@param oldname string
---@param newname string
---@return true|nil result
---@return string err
function os.rename(oldname, newname) end

---
--- Sets the current locale of the program. `locale` is a system-dependent
--- string specifying a locale; `category` is an optional string describing
--- which category to change: `"all"`, `"collate"`, `"ctype"`, `"monetary"`,
--- `"numeric"`, or `"time"`; the default category is `"all"`. The function
--- returns the name of the new locale, or **nil** if the request cannot be
--- honored.
---
--- If `locale` is the empty string, the current locale is set to an
--- implementation-defined native locale. If `locale` is the string "`C`",
--- the current locale is set to the standard C locale.
---
--- When called with **nil** as the first argument, this function only returns
--- the name of the current locale for the given category.
---
--- This function may be not thread safe because of its reliance on C function
--- `setlocale`.
---@param locale string
---@param category? string
---@return string|nil
function os.setlocale(locale, category) end

---@class std.osdateparam
---@field year integer|string four digits
---@field month integer|string 1-12
---@field day integer|string 1-31
---@field hour (integer|string)? 0-23
---@field min (integer|string)? 0-59
---@field sec (integer|string)? 0-61, due to leap seconds
---@field wday (integer|string)? 1-7, Sunday is 1
---@field yday (integer|string)? 1-366
---@field isdst boolean? daylight saving flag, a boolean.

---
--- Returns the current time when called without arguments, or a time
--- representing the date and time specified by the given table. This table
--- must have fields `year`, `month`, and `day`, and may have fields `hour`
--- (default is 12), `min` (default is 0), `sec` (default is 0), and `isdst`
--- (default is **nil**). Other fields are ignored. For a description of these
--- fields, see the `os.date` function.
---
--- When the function is called, the values in these fields do not need to be
--- inside their valid ranges. For instance, if `sec` is -10, it means 10 seconds
--- before the time specified by the other fields; if `hour` is 1000, it means
--- 1000 hours after the time specified by the other fields.
---
--- The returned value is a number, whose meaning depends on your system. In
--- POSIX, Windows, and some other systems, this number counts the number of
--- seconds since some given start time (the "epoch"). In other systems, the
--- meaning is not specified, and the number returned by `time` can be used only
--- as an argument to `os.date` and `os.difftime`.
---
--- When called with a table, `os.time` also normalizes all the fields
--- documented in the `os.date` function, so that they represent the same time
--- as before the call but with values inside their valid ranges.
---@param date? std.osdateparam
---@return integer
function os.time(date) end

---
--- Returns a string with a file name that can be used for a temporary
--- file. The file must be explicitly opened before its use and explicitly
--- removed when no longer needed.
---
--- On some systems (POSIX), this function also creates a file with that
--- name, to avoid security risks. (Someone else might create the file with
--- wrong permissions in the time between getting the name and creating the
--- file.) You still have to open the file to use it and to remove it (even
--- if you do not use it).
---
--- When possible, you may prefer to use `io.tmpfile`, which automatically
--- removes the file when the program ends.
---@return string
function os.tmpname() end

---
--- **xmake extension**
--- Check if the file or directory exists
---@param path string
---@return boolean
function os.exists(path) end

---
--- **xmake extension**
--- Check if path is a file
---@param path string
---@return boolean
function os.isfile(path) end

---
--- **xmake extension**
--- Check if path is a directory
---@param path string
---@return boolean
function os.isdir(path) end

---
--- **xmake extension**
--- Check if path is a symbolic link
---@param path string
---@return boolean
function os.islink(path) end

---
--- **xmake extension**
--- Check if path is an executable program (checks common suffixes on Windows).
---@param path string
---@return boolean
function os.isexec(path) end

---
--- **xmake extension**
--- Create directory
---@param dir string
---@return boolean
function os.mkdir(dir) end

---
--- **xmake extension**
--- Remove directory recursively
---@param dir string
---@return boolean
function os.rmdir(dir) end

---
--- **xmake extension**
--- Copy file or directory
---@param src string
---@param dst string
---@return boolean
function os.cp(src, dst) end

---
--- **xmake extension**
--- Move/rename file or directory
---@param src string
---@param dst string
---@return boolean
function os.mv(src, dst) end

---
--- **xmake extension**
--- Remove file or directory
---@param path string
---@return boolean
function os.rm(path) end

---
--- **xmake extension**
--- Create or update file timestamp (like touch). Creates parent dirs if needed.
---@param path string
---@param opt? table
---@return boolean
function os.touch(path, opt) end

---
--- **xmake extension**
--- Get file size
---@param path string
---@return integer|nil
function os.filesize(path) end

---
--- **xmake extension**
--- Get modification time
---@param path string
---@return integer|nil
function os.mtime(path) end

---
--- **xmake extension**
--- Monotonic clock in milliseconds.
---@return integer
function os.mclock() end

---
--- **xmake extension**
--- Get current working directory
---@return string
function os.curdir() end

---
--- **xmake extension**
--- Get temporary directory
---@return string
function os.tmpdir() end

---
--- **xmake extension**
--- Get a unique temporary file path.
---@param key? string
---@param opt? table
---@return string
function os.tmpfile(key, opt) end

---
--- **xmake extension**
--- Get program directory
---@return string
function os.programdir() end

---
--- **xmake extension**
--- Get program file path
---@return string
function os.programfile() end

---
--- **xmake extension**
--- Get xmake working directory (internal runtime cwd).
---@return string
function os.workingdir() end

---
--- **xmake extension**
--- Get script directory
---@return string
function os.scriptdir() end

---
--- **xmake extension**
--- Get xmake version string, e.g. "2.9.5"
---@return string
function os.xmakever() end

---
--- **xmake extension**
--- Get host operating system
---@return string
function os.host() end

---
--- **xmake extension**
--- Get host architecture
---@return string
function os.arch() end

---
--- **xmake extension**
--- Get sub-host information
---@return string
function os.subhost() end

---
--- **xmake extension**
--- Get sub-architecture information
---@return string
function os.subarch() end

---
--- **xmake extension**
--- Change directory
---@param dir string
---@return boolean
function os.cd(dir) end

---
--- **xmake extension**
--- Execute command and return output
---@param cmd string
---@param opt? table
---@return boolean ok
---@return string|nil stdout
---@return string|nil stderr
---@return string? errors
function os.iorun(cmd, opt) end

---
--- **xmake extension**
--- Execute command and return output with environment
---@param program string
---@param argv table
---@param opt? table
---@return boolean ok
---@return string|nil stdout
---@return string|nil stderr
---@return string? errors
function os.iorunv(program, argv, opt) end

---
--- **xmake extension**
--- Run command and return exit code
---@param cmd string
---@return boolean ok
---@return string? errors
function os.run(cmd) end

---
--- **xmake extension**
--- Run command with arguments and return exit code
---@param program string
---@param argv table
---@param opt? table
---@return boolean ok
---@return string? errors
function os.runv(program, argv, opt) end

---
--- **xmake extension**
--- Execute command and return output
---@param cmd string
---@return integer|nil code
---@return string? errors
function os.exec(cmd) end

---
--- **xmake extension**
--- Execute command with arguments and return output
---@param program string
---@param argv table
---@param opt? table
---@return integer|nil code
---@return string? errors
function os.execv(program, argv, opt) end

---
--- **xmake extension**
--- Verbosely run command (print command before running)
---@param cmd string
---@return boolean ok
---@return string? errors
function os.vrun(cmd) end

---
--- **xmake extension**
--- Verbosely run program with arguments
---@param program string
---@param argv table
---@param opt? table
---@return boolean ok
---@return string? errors
function os.vrunv(program, argv, opt) end

---
--- **xmake extension**
--- Verbosely execute command and return exit code
---@param cmd string
---@return integer|nil code
---@return string? errors
function os.vexec(cmd) end

---
--- **xmake extension**
--- Verbosely execute program with arguments and return exit code
---@param program string
---@param argv table
---@param opt? table
---@return integer|nil code
---@return string? errors
function os.vexecv(program, argv, opt) end

---
--- **xmake extension**
--- Find executable program in PATH
---@param name string
---@return string|nil
function os.which(name) end

---
--- **xmake extension**
--- List directory contents
---@param dir string
---@param recursively? boolean
---@param pattern? string
---@return string[]
function os.dirs(dir, recursively, pattern) end

---
--- **xmake extension**
--- List files in directory
---@param dir string
---@param recursively? boolean
---@param pattern? string
---@return string[]
function os.files(dir, recursively, pattern) end

---
--- **xmake extension**
--- Raise an exception
---@param message string
---@param level? integer
function os.raise(message, level) end

---
--- **xmake extension**
--- Raise an exception with stack level offset. Prefer `os.raise` in user code.
---@param level integer
---@param message string
---@param ... any
function os.raiselevel(level, message, ...) end

---
--- **xmake extension**
--- Try to execute function and catch exception
---@param func function
---@return any, string?
function os.trybool(func) end

---
--- **xmake extension**
--- Sleep for specified time
---@param ms integer milliseconds
function os.sleep(ms) end

---
--- **xmake extension**
--- Get environment variable with default value
---@param name string
---@param default? string
---@return string|nil
---@overload fun():table
function os.getenvs(name, default) end

---
--- **xmake extension**
--- Set environment variable
---@param name string
---@param value string
---@return boolean
function os.setenv(name, value) end

---
--- **xmake extension**
--- Add one value to an environment variable like PATH
---@param name string
---@param value string
---@return boolean
function os.addenv(name, value) end

---
--- **xmake extension**
--- Add multiple values to an environment variable like PATH
---@param name string
---@param values string|string[]
---@return boolean
function os.addenvs(name, values) end

---
--- **xmake extension**
--- Set multiple environment variables
---@param envs table<string,string>
---@return boolean
function os.setenvs(envs) end

---
--- **xmake extension**
--- Generate UUID
---@return string
function os.uuid() end

---
--- **xmake extension**
--- Generate UUID with specified format
---@param name string
---@return string
function os.uuid4(name) end
---
--- **xmake extension**
--- Read value of a symbolic link
---@param path string
---@return string|nil
function os.readlink(path) end

---
--- **xmake extension**
--- Check whether file system is case-sensitive at given path
---@param path string
---@return boolean
function os.fscase(path) end

---
--- **xmake extension**
--- Create a symbolic link or hard link based on options
---@param src string
---@param dst string
---@param opt? table  options: {force:boolean, hard:boolean}
---@return boolean
function os.ln(src, dst, opt) end

---
--- **xmake extension**
--- Get program/project directories and files.
---@return string
function os.projectdir() end
---@return string
function os.projectfile() end


return os
