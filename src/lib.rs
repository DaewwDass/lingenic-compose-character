//! character: CJK font primitives using pen-path and brush-path.
//!
//! This crate provides types for defining CJK (Chinese, Japanese, Korean)
//! fonts using stroke-based primitives. It parallels the `lettering` crate
//! for Western fonts, but supports:
//!
//! - **Multiple scripts**: Han (漢字), Kana (ひらがな/カタカナ), Hangul (한글)
//! - **Regional variants**: CN, TW, JP, KR forms of the same character
//! - **Calligraphic styles**: Using brush-path for 楷書, 行書, 草書
//! - **Typographic styles**: Using pen-path for 宋體, 黑體
//! - **Stroke semantics**: Entry/exit styles (入鋒/收鋒), actions (頓提折), connections (筆勢)
//!
//! ## Architecture
//!
//! ```text
//! pen-path    ─┬─►  lettering   (Western fonts)
//!              │
//! brush-path  ─┴─►  character   (CJK fonts)
//! ```
//!
//! ## Choosing Brush vs Pen
//!
//! | Font Style | Primitives | Notes |
//! |------------|------------|-------|
//! | 楷書 (Kai) | brush-path | Calligraphic, pressure variation (輕重) |
//! | 行書 (Xing) | brush-path | Running script, connected strokes |
//! | 草書 (Cao) | brush-path | Grass script, highly connected (牽絲) |
//! | 宋體 (Song) | pen-path | Typographic, constructed strokes |
//! | 黑體 (Hei) | pen-path | Sans-serif, uniform strokes |
//!
//! ## Example
//!
//! ```
//! use character::{CjkFont, Character, CharacterStroke, BrushDef, Region, Script};
//! use brush_path::{Brush, BrushStroke, StrokeKind};
//!
//! // Create a font
//! let mut font = CjkFont::new("MyKaiFont")
//!     .with_script(Script::Han)
//!     .with_region(Region::CN);
//!
//! // Define a brush
//! font.add_brush(BrushDef::medium("main"));
//!
//! // Create a character (永 - eternal, the classic example)
//! let mut yong = Character::new(0x6C38);
//!
//! // Add strokes (simplified - real character has 5 strokes)
//! let stroke = BrushStroke::new(
//!     font.get_brush("main").unwrap().brush.clone(),
//!     StrokeKind::Dian,
//! );
//! yong.add_stroke(CharacterStroke::brush("main", StrokeKind::Dian, stroke));
//!
//! font.add_character(yong);
//!
//! assert_eq!(font.character_count(), 1);
//! ```

pub mod region;
pub mod script;
pub mod brush;
pub mod stroke;
pub mod character;
pub mod font;

// Re-exports: core types
pub use region::Region;
pub use script::Script;
pub use brush::BrushDef;
pub use stroke::{CharacterStroke, BrushStrokeData, PenStrokeData};
pub use character::Character;
pub use font::{CjkFont, CjkFontMetrics};

// Re-export primitives from dependencies for convenience
pub use brush_path::{
    Brush, BristleType, BrushState,
    BrushStroke, BrushSegment, BrushPoint, BrushAction,
    StrokeKind, EntryStyle, ExitStyle, TipMode, ConnectionStyle,
    Ink, Paper,
};
pub use pen_path::{Point, Cp, Color, Pen, Path, Connector};
