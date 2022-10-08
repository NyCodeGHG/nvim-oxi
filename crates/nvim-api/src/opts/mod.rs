//! Contains the `*Opts` structs representing the optional arguments
//! passsed to Neovim API functions.

mod buf_attach;
mod buf_delete;
mod clear_autocmds;
mod cmd;
mod create_augroup;
mod create_autocmd;
mod create_command;
mod decoration_provider;
mod eval_statusline;
mod exec_autocmds;
mod get_autocmds;
mod get_commands;
mod get_context;
mod get_extmark_by_id;
mod get_extmarks;
mod get_mark;
mod get_option_value;
mod get_text;
mod notify;
mod open_term;
#[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
mod parse_cmd;
mod select_popup_menu_item;
mod set_extmark;
mod set_highlight;
mod set_keymap;

pub use buf_attach::*;
pub use buf_delete::*;
pub use clear_autocmds::*;
pub use cmd::*;
pub use create_augroup::*;
pub use create_autocmd::*;
pub use create_command::*;
pub use decoration_provider::*;
pub use eval_statusline::*;
pub use exec_autocmds::*;
pub use get_autocmds::*;
pub use get_commands::*;
pub use get_context::*;
pub use get_extmark_by_id::*;
pub use get_extmarks::*;
pub use get_mark::*;
pub use get_option_value::*;
pub use get_text::*;
pub use notify::*;
pub use open_term::*;
#[cfg(any(feature = "neovim-0-8", feature = "neovim-nightly"))]
pub use parse_cmd::*;
pub use select_popup_menu_item::*;
pub use set_extmark::*;
pub use set_highlight::*;
pub use set_keymap::*;
