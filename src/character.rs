//! Character structure — the CJK equivalent of a glyph.
//!
//! A Character represents a single CJK character with its strokes,
//! stroke order, and regional variant information.

use crate::region::Region;
use crate::script::Script;
use crate::stroke::CharacterStroke;

/// A character in a CJK font.
///
/// Unlike Western glyphs which are simple stroke collections,
/// CJK characters have:
/// - Defined stroke order
/// - Stroke classifications (横, 竖, 撇, etc.)
/// - Regional variants
/// - Script type (Han, Kana, Hangul)
#[derive(Debug, Clone)]
pub struct Character {
    /// Unicode codepoint.
    pub codepoint: u32,

    /// Character name (usually the character itself or description).
    pub name: String,

    /// Script type.
    pub script: Script,

    /// Primary region (for regional variants).
    pub region: Option<Region>,

    /// Advance width (in font units).
    pub advance_width: i64,

    /// Strokes in stroke order.
    pub strokes: Vec<CharacterStroke>,

    /// Vertical advance (for vertical writing).
    pub vertical_advance: Option<i64>,

    /// Left side bearing.
    pub lsb: Option<i64>,

    /// Top side bearing (for vertical writing).
    pub tsb: Option<i64>,
}

impl Character {
    /// Create a new character.
    pub fn new(codepoint: u32) -> Self {
        let name = char::from_u32(codepoint)
            .map(|c| c.to_string())
            .unwrap_or_else(|| format!("U+{:04X}", codepoint));

        Self {
            codepoint,
            name,
            script: Script::Han,
            region: None,
            advance_width: 1000, // Default full-width
            strokes: Vec::new(),
            vertical_advance: None,
            lsb: None,
            tsb: None,
        }
    }

    /// Create a character with a specific script.
    pub fn with_script(mut self, script: Script) -> Self {
        self.script = script;
        self
    }

    /// Set the region.
    pub fn with_region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    /// Set advance width.
    pub fn with_advance_width(mut self, width: i64) -> Self {
        self.advance_width = width;
        self
    }

    /// Set vertical advance.
    pub fn with_vertical_advance(mut self, advance: i64) -> Self {
        self.vertical_advance = Some(advance);
        self
    }

    /// Add a stroke (appended in stroke order).
    pub fn add_stroke(&mut self, stroke: CharacterStroke) {
        self.strokes.push(stroke);
    }

    /// Set all strokes.
    pub fn with_strokes(mut self, strokes: Vec<CharacterStroke>) -> Self {
        self.strokes = strokes;
        self
    }

    /// Get the character this represents.
    pub fn char(&self) -> Option<char> {
        char::from_u32(self.codepoint)
    }

    /// Number of strokes.
    pub fn stroke_count(&self) -> usize {
        self.strokes.len()
    }

    /// Check if character is empty (no strokes).
    pub fn is_empty(&self) -> bool {
        self.strokes.is_empty()
    }

    /// Whether this character uses brush strokes.
    pub fn uses_brush(&self) -> bool {
        self.strokes.iter().any(|s| s.is_brush())
    }

    /// Whether this character uses pen strokes.
    pub fn uses_pen(&self) -> bool {
        self.strokes.iter().any(|s| s.is_pen())
    }

    /// Whether this is a full-width character.
    pub fn is_fullwidth(&self) -> bool {
        // Typically 1000 units in a 1000-unit em
        self.advance_width >= 900
    }

    /// Whether this is a half-width character.
    pub fn is_halfwidth(&self) -> bool {
        self.advance_width <= 600
    }

    /// Get vertical advance (defaults to advance_width for square characters).
    pub fn get_vertical_advance(&self) -> i64 {
        self.vertical_advance.unwrap_or(self.advance_width)
    }
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(c) = self.char() {
            write!(f, "{}", c)
        } else {
            write!(f, "U+{:04X}", self.codepoint)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_creation() {
        let ch = Character::new(0x6C38); // 永
        assert_eq!(ch.codepoint, 0x6C38);
        assert_eq!(ch.char(), Some('永'));
        assert_eq!(ch.script, Script::Han);
    }

    #[test]
    fn test_character_with_region() {
        let ch = Character::new(0x76F4) // 直
            .with_region(Region::JP);
        assert_eq!(ch.region, Some(Region::JP));
    }

    #[test]
    fn test_character_display() {
        let ch = Character::new(0x6C38);
        assert_eq!(format!("{}", ch), "永");
    }

    #[test]
    fn test_fullwidth_halfwidth() {
        let full = Character::new(0x6C38).with_advance_width(1000);
        assert!(full.is_fullwidth());
        assert!(!full.is_halfwidth());

        let half = Character::new(0x0041).with_advance_width(500);
        assert!(!half.is_fullwidth());
        assert!(half.is_halfwidth());
    }
}
