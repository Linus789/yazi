#![allow(
	clippy::if_same_then_else,
	clippy::len_without_is_empty,
	clippy::module_inception,
	clippy::option_map_unit_fn,
	clippy::unit_arg
)]

mod blocker;
mod event;
pub mod external;
pub mod files;
pub mod help;
mod highlighter;
pub mod input;
pub mod manager;
pub mod position;
pub mod select;
pub mod tasks;
pub mod which;

pub use blocker::*;
pub use event::*;
pub use highlighter::*;
pub use position::*;

pub fn init() { init_blocker(); }
