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
mod echo;
mod eval_statusline;
#[cfg(feature = "neovim-nightly")]
mod exec;
mod exec_autocmds;
mod get_autocmds;
mod get_commands;
mod get_context;
mod get_extmark_by_id;
mod get_extmarks;
#[cfg(feature = "neovim-nightly")]
mod get_highlight;
mod get_mark;
#[cfg(feature = "neovim-nightly")]
mod get_namespace;
mod get_text;
mod notify;
mod open_term;
mod option;
mod parse_cmd;
mod select_popup_menu_item;
mod set_extmark;
mod set_highlight;
mod set_keymap;
mod set_mark;
#[cfg(feature = "neovim-nightly")]
mod win_text_height;

pub use buf_attach::*;
pub use buf_delete::*;
pub use clear_autocmds::*;
pub use cmd::*;
pub use create_augroup::*;
pub use create_autocmd::*;
pub use create_command::*;
pub use decoration_provider::*;
pub use echo::*;
pub use eval_statusline::*;
#[cfg(feature = "neovim-nightly")]
pub use exec::*;
pub use exec_autocmds::*;
pub use get_autocmds::*;
pub use get_commands::*;
pub use get_context::*;
pub use get_extmark_by_id::*;
pub use get_extmarks::*;
#[cfg(feature = "neovim-nightly")]
pub use get_highlight::*;
pub use get_mark::*;
#[cfg(feature = "neovim-nightly")]
pub use get_namespace::*;
pub use get_text::*;
pub use notify::*;
pub use open_term::*;
pub use option::*;
pub use parse_cmd::*;
pub use select_popup_menu_item::*;
pub use set_extmark::*;
pub use set_highlight::*;
pub use set_keymap::*;
pub use set_mark::*;
#[cfg(feature = "neovim-nightly")]
pub use win_text_height::*;
