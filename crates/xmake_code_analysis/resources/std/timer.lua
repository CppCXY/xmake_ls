---@meta
-- Minimal timer helpers for xmake scripts.

---@class timerlib
timer = {}

---@class timer_handle
local timer_handle = {}

--- Stop this timer.
function timer_handle:stop() end

--- Start a periodic timer with interval ms.
---@param interval integer
---@param cb fun()
---@return timer_handle
function timer.start(interval, cb) end

--- Run callback once after delay ms.
---@param delay integer
---@param cb fun()
---@return timer_handle
function timer.after(delay, cb) end

--- Stop all timers (best effort for scripting use).
function timer.stop() end
