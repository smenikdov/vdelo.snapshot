local static = require("vdelosnap.static")
local visual_utils = require("vdelosnap.utils.visual")
local table_utils = require("vdelosnap.utils.table")
local string_utils = require("vdelosnap.utils.string")
local config_module = require("vdelosnap.config")
local highlight_module = require("vdelosnap.highlight")

local main = {
    cwd = static.cwd,
    preview_switch = static.preview_switch,
    highlight_mode_config = nil,
}

function main.setup(config)
    static.config = table_utils.merge(static.config, config == nil and {} or config)
end

function main.copy_into_clipboard_with_config(config)
    require("generator").copy_into_clipboard(config)
    vim.cmd("delmarks <>")
    vim.notify("Save snapshot into clipboard successfully")
end

function main.copy_ascii_snapshot(extension)
    require("generator").copy_ascii(config_module.get_config(extension))
    vim.cmd("delmarks <>")
    vim.notify("Save snapshot into clipboard successfully")
end

function main.save_snapshot_with_config(config)
    if string_utils.is_str_empty(static.config.save_path) then
        error(
            "If you want to save snapshot in somewhere, you should config the save_path before, refer: https://github.com/mistricky/codesnap.nvim?tab=readme-ov-file#save-the-snapshot",
            0
        )
    end

    local matched_extension = string.match(static.config.save_path, "%.(.+)$")

    if matched_extension ~= "png" and matched_extension ~= nil then
        error("The extension of save_path should be .png", 0)
    end

    require("generator").save_snapshot(config)
    vim.cmd("delmarks <>")
    vim.notify("Save snapshot in " .. config.save_path .. " successfully")
end

function main.copy_into_clipboard(extension)
    main.copy_into_clipboard_with_config(config_module.get_config(extension))
end

function main.save_snapshot(extension)
    main.save_snapshot_with_config(config_module.get_config(extension))
end

function main.highlight_mode_copy_into_clipboard(extension)
    main.highlight_mode_config = config_module.get_config(extension)

    highlight_module.create_highlight_selector_window(
        "copy_into_clipboard_with_config",
        visual_utils.get_selected_lines()
    )
end

function main.highlight_mode_save_snapshot(extension)
    main.highlight_mode_config = config_module.get_config(extension)

    highlight_module.create_highlight_selector_window("save_snapshot_with_config", visual_utils.get_selected_lines())
end

return main
