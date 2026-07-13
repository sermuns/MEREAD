local M = {}

local VERSION = "v1.0.0"

local library_filename = "meread-" .. VERSION .. ".so"
local meread_data_dir = vim.fs.joinpath(vim.fn.stdpath("data"), "meread")
local lib_path = vim.fs.joinpath(meread_data_dir, library_filename)

local function download(callback)
	vim.fn.mkdir(meread_data_dir, "p")

	local url = string.format("https://github.com/sermuns/meread/releases/download/%s/%s", VERSION, library_filename)
	vim.notify("[MEREAD] downloading prebuilt binary...", vim.log.levels.INFO)

	local obj = vim.system({ "curl", "-LO", "--output-dir", meread_data_dir, url }, {}):wait()

	vim.schedule(function()
		if obj.code == 0 then
			vim.notify("[MEREAD] update complete.", vim.log.levels.INFO)
			callback()
		else
			vim.notify("[MEREAD] download failed!", vim.log.levels.ERROR)
		end
	end)
end

local function load_library()
	local loader, err = package.loadlib(lib_path, "luaopen_meread")
	if not loader then
		error("[MEREAD] Failed to load binary: " .. tostring(err))
	end

	loader().setup {}
end

function M.setup(opts)
	if vim.fn.filereadable(lib_path) == 0 then
		download(load_library)
	else
		load_library()
	end
end

return M
