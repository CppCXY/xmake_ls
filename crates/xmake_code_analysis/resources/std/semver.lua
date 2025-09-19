---@meta
-- Minimal semver helpers matching common xmake usage.

---@class semverlib
semver = {}

--- Compare two versions. Return -1 if a<b, 0 if equal, 1 if a>b.
---@param a string
---@param b string
---@return integer
function semver.compare(a, b) end

--- True if version `v` satisfies range expression `range` (e.g. ">=1.2,<2.0").
---@param v string
---@param range string
---@return boolean
function semver.satisfies(v, range) end

--- Normalize version string (e.g. trim/expand to x.y.z).
---@param v string
---@return string
function semver.normalize(v) end
