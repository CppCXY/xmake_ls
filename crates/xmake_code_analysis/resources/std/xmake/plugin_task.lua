---@meta
---[plugin_task](https://xmake.io/api/description/plugin-and-task)

---
---Defining plugins or tasks
---
---[Open in browser](https://xmake.io/api/description/plugin-and-task#task)
--- Example:
--- ```lua
--- task("hello")
---     on_run(function () print("hello") end)
--- task_end()
--- ```
---@scope task
---@param name string Task name
---@return nil
function task(name) end

---
---End defining plugins or tasks
---
---[Open in browser](https://xmake.io/api/description/plugin-and-task#task_end)
---
---@return nil
function task_end() end

---
---Setting the task menu
---
---[Open in browser](https://xmake.io/api/description/plugin-and-task#set_menu)
---@scope task
---@param menu TaskMenu Task menu
---@return nil
function set_menu(menu) end

---
---Setting task categories
---
---[Open in browser](https://xmake.io/api/description/plugin-and-task#set_category)
---@scope task
---@param name string|"plugin"|"action" Category name
---@return nil
function set_category(name) end

---
---Setting up a task to run a script
---
---[Open in browser](https://xmake.io/api/description/plugin-and-task#on_run)
---@scope task
---@param func fun(): nil Function to run when task is running
---@return nil
function on_run(func) end

---
---Setting up a task to run a script
---
---[Open in browser](https://xmake.io/api/description/plugin-and-task#on_run)
---@scope task
---@param name string Script name to run when task is running
---@return nil
function on_run(name) end
