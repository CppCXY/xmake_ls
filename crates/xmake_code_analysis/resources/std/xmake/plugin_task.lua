---@meta
---[plugin_task](https://xmake.io/#/manual/plugin_task)

---
---Defining plugins or tasks
---
---[Open in browser](https://xmake.io/#/manual/plugin_task?id=task)
---
---@param name string Task name
---@return nil
function task(name) end

---
---End defining plugins or tasks
---
---[Open in browser](https://xmake.io/#/manual/plugin_task?id=task_end)
---
---@return nil
function task_end() end

---
---**Scoped: task**
---
---Setting the task menu
---
---[Open in browser](https://xmake.io/#/manual/plugin_task?id=taskset_menu)
---
---@param menu TaskMenu Task menu
---@return nil
function set_menu(menu) end

---
---**Scoped: task**
---
---Setting task categories
---
---[Open in browser](https://xmake.io/#/manual/plugin_task?id=taskset_category)
---
---@param name string|"plugin"|"action" Category name
---@return nil
function set_category(name) end

---
---**Scoped: task**
---
---Setting up a task to run a script
---
---[Open in browser](https://xmake.io/#/manual/plugin_task?id=taskon_run)
---
---@param func fun(): nil Function to run when task is running
---@return nil
function on_run(func) end

---
---**Scoped: task**
---
---Setting up a task to run a script
---
---[Open in browser](https://xmake.io/#/manual/plugin_task?id=taskon_run)
---
---@param name string Script name to run when task is running
---@return nil
function on_run(name) end

---
---Run a task
---
---[Open in browser](https://xmake.io/#/manual/plugin_task?id=task-run)
---
---@param name string Task name
---@param args_menu? table Args for task menu option
---@param ... any Args for task entry main
---@return nil
function task.run(name, args_menu, ...) end
