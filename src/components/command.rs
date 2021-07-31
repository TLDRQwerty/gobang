use crate::config::KeyConfig;

static CMD_GROUP_GENERAL: &str = "-- General --";
static CMD_GROUP_TABLE: &str = "-- Table --";
static CMD_GROUP_DATABASES: &str = "-- Databases --";

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct CommandText {
    pub name: String,
    pub group: &'static str,
    pub hide_help: bool,
}

impl CommandText {
    pub const fn new(name: String, group: &'static str) -> Self {
        Self {
            name,
            group,
            hide_help: false,
        }
    }
}

pub struct CommandInfo {
    pub text: CommandText,
}

impl CommandInfo {
    pub const fn new(text: CommandText) -> Self {
        Self { text }
    }
}

pub fn scroll(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Scroll up/down/left/right [{},{},{},{}]",
            key.scroll_up.to_string(),
            key.scroll_down.to_string(),
            key.scroll_left.to_string(),
            key.scroll_right.to_string()
        ),
        CMD_GROUP_GENERAL,
    )
}

pub fn scroll_to_top_bottom(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Scroll to top/bottom [{},{}]",
            key.scroll_to_top.to_string(),
            key.scroll_to_bottom.to_string(),
        ),
        CMD_GROUP_GENERAL,
    )
}

pub fn expand_collapse(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Expand/Collapse [{},{}]",
            key.scroll_right.to_string(),
            key.scroll_left.to_string(),
        ),
        CMD_GROUP_DATABASES,
    )
}

pub fn filter(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!("Filter [{}]", key.filter.to_string()),
        CMD_GROUP_GENERAL,
    )
}

pub fn move_focus(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Move focus to left/right [{},{}]",
            key.focus_left, key.focus_right
        ),
        CMD_GROUP_GENERAL,
    )
}

pub fn extend_selection_by_one_cell(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Extend selection by one cell up/down/left/right [{},{},{},{}]",
            key.extend_selection_by_one_cell_up,
            key.extend_selection_by_one_cell_down,
            key.extend_selection_by_one_cell_left,
            key.extend_selection_by_one_cell_right
        ),
        CMD_GROUP_TABLE,
    )
}

pub fn tab_records(key: &KeyConfig) -> CommandText {
    CommandText::new(format!("Records [{}]", key.tab_records), CMD_GROUP_TABLE)
}

pub fn tab_structure(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!("Structure [{}]", key.tab_structure),
        CMD_GROUP_TABLE,
    )
}

pub fn toggle_tabs(key_config: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Tab [{},{}]",
            key_config.tab_records, key_config.tab_structure
        ),
        CMD_GROUP_GENERAL,
    )
}

pub fn help(key_config: &KeyConfig) -> CommandText {
    CommandText::new(
        format!("Help [{}]", key_config.open_help),
        CMD_GROUP_GENERAL,
    )
}