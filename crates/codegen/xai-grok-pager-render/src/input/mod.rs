pub mod key;
pub mod keyboard_normalizer;
<<<<<<< HEAD:crates/codegen/xai-grok-pager/src/input/mod.rs
pub(crate) mod line_editor;
=======
pub mod line_editor;
>>>>>>> a28ee2b2063426e8816e380ccea528b9de95e5da:crates/codegen/xai-grok-pager-render/src/input/mod.rs
#[cfg(target_os = "macos")]
pub mod macos_modifiers;
pub mod mouse;
pub(crate) mod scroll_log;
pub mod terminal_support;

pub use keyboard_normalizer::{KeyboardNormalizer, ModifierState};
pub use terminal_support::{
    is_apple_terminal_newline_modifier_held, is_delivered_super_enter, is_mod_enter,
    os_modifier_rescue_suppressed, suppress_os_modifier_rescue,
};
