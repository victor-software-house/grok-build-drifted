pub mod key;
pub mod keyboard_normalizer;
<<<<<<< HEAD:crates/codegen/xai-grok-pager/src/input/mod.rs
pub(crate) mod line_editor;
=======
pub mod line_editor;
>>>>>>> bb7f39d5858cbf5e00de639367f59debbdcb0138:crates/codegen/xai-grok-pager-render/src/input/mod.rs
#[cfg(target_os = "macos")]
pub mod macos_modifiers;
pub mod mouse;
pub(crate) mod scroll_log;
pub mod terminal_support;

pub use keyboard_normalizer::{KeyboardNormalizer, ModifierState};
pub use terminal_support::{is_apple_terminal_newline_modifier_held, is_mod_enter};
