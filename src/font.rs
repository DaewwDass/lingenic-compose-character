//! CJK font structure and metrics.

use std::collections::HashMap;

use brush_path::{Ink, Paper};
use pen_path::Pen;

use crate::brush::BrushDef;
use crate::character::Character;
use crate::region::Region;
use crate::script::Script;

/// A CJK font.
///
/// Contains characters, brush/pen definitions, and font metadata.
/// Can contain multiple scripts (Han, Kana, Hangul) and regional variants.
#[derive(Debug, Clone)]
pub struct CjkFont {
    /// Font name.
    pub name: String,

    /// Font metrics.
    pub metrics: CjkFontMetrics,

    /// Primary script.
    pub script: Script,

    /// Primary region.
    pub region: Region,

    /// Named brush definitions.
    pub brushes: HashMap<String, BrushDef>,

    /// Named pen definitions (for typographic styles).
    pub pens: HashMap<String, Pen>,

    /// Named ink definitions.
    pub inks: HashMap<String, Ink>,

    /// Source paper aesthetic.
    pub paper: Option<Paper>,

    /// Characters indexed by codepoint.
    pub characters: HashMap<u32, Character>,

    /// Regional variant overrides: (codepoint, region) -> Character.
    pub variants: HashMap<(u32, Region), Character>,
}

/// CJK font metrics.
///
/// CJK fonts typically use a square em with uniform advance width.
#[derive(Debug, Clone, Copy, Default)]
pub struct CjkFontMetrics {
    /// Units per em (typically 1000 or 2048).
    pub units_per_em: i64,

    /// Ascender (top of em square).
    pub ascender: i64,

    /// Descender (bottom of em square, typically negative or zero).
    pub descender: i64,

    /// Ideographic em square side (usually equals units_per_em).
    pub ideographic_em: i64,

    /// Default advance width for fullwidth characters.
    pub fullwidth_advance: i64,

    /// Default advance width for halfwidth characters.
    pub halfwidth_advance: i64,

    /// Line gap for horizontal text.
    pub line_gap: i64,

    /// Column gap for vertical text.
    pub column_gap: i64,
}

impl CjkFont {
    /// Create a new CJK font.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            metrics: CjkFontMetrics::default(),
            script: Script::Han,
            region: Region::CN,
            brushes: HashMap::new(),
            pens: HashMap::new(),
            inks: HashMap::new(),
            paper: None,
            characters: HashMap::new(),
            variants: HashMap::new(),
        }
    }

    /// Set primary script.
    pub fn with_script(mut self, script: Script) -> Self {
        self.script = script;
        self
    }

    /// Set primary region.
    pub fn with_region(mut self, region: Region) -> Self {
        self.region = region;
        self
    }

    /// Add a brush definition.
    pub fn add_brush(&mut self, brush: BrushDef) {
        self.brushes.insert(brush.name.clone(), brush);
    }

    /// Get a brush by name.
    pub fn get_brush(&self, name: &str) -> Option<&BrushDef> {
        self.brushes.get(name)
    }

    /// Add a pen definition.
    pub fn add_pen(&mut self, name: &str, pen: Pen) {
        self.pens.insert(name.to_string(), pen);
    }

    /// Get a pen by name.
    pub fn get_pen(&self, name: &str) -> Option<&Pen> {
        self.pens.get(name)
    }

    /// Add an ink definition.
    pub fn add_ink(&mut self, name: &str, ink: Ink) {
        self.inks.insert(name.to_string(), ink);
    }

    /// Get an ink by name.
    pub fn get_ink(&self, name: &str) -> Option<&Ink> {
        self.inks.get(name)
    }

    /// Set source paper aesthetic.
    pub fn with_paper(mut self, paper: Paper) -> Self {
        self.paper = Some(paper);
        self
    }

    /// Add a character.
    pub fn add_character(&mut self, character: Character) {
        self.characters.insert(character.codepoint, character);
    }

    /// Add a regional variant.
    pub fn add_variant(&mut self, character: Character, region: Region) {
        self.variants.insert((character.codepoint, region), character);
    }

    /// Get a character by codepoint.
    pub fn get_character(&self, codepoint: u32) -> Option<&Character> {
        self.characters.get(&codepoint)
    }

    /// Get a character for a specific region, falling back to default.
    pub fn get_character_for_region(&self, codepoint: u32, region: Region) -> Option<&Character> {
        // Try regional variant first
        self.variants.get(&(codepoint, region))
            .or_else(|| self.characters.get(&codepoint))
    }

    /// Get a character by char.
    pub fn get_character_for_char(&self, c: char) -> Option<&Character> {
        self.characters.get(&(c as u32))
    }

    /// Number of characters.
    pub fn character_count(&self) -> usize {
        self.characters.len()
    }

    /// Number of regional variants.
    pub fn variant_count(&self) -> usize {
        self.variants.len()
    }

    /// Convert font units to centipoints at a given size.
    pub fn to_cp(&self, font_units: i64, size_cp: i64) -> i64 {
        if self.metrics.units_per_em == 0 {
            return font_units;
        }
        font_units * size_cp / self.metrics.units_per_em
    }

    /// Get all unique scripts in this font.
    pub fn scripts(&self) -> Vec<Script> {
        let mut scripts: Vec<Script> = self.characters.values()
            .map(|c| c.script)
            .collect();
        scripts.sort_by_key(|s| s.code());
        scripts.dedup();
        scripts
    }
}

impl CjkFontMetrics {
    /// Create metrics with units per em.
    pub fn new(units_per_em: i64) -> Self {
        Self {
            units_per_em,
            ascender: (units_per_em * 88) / 100,  // 88% typical
            descender: -(units_per_em * 12) / 100, // -12% typical
            ideographic_em: units_per_em,
            fullwidth_advance: units_per_em,
            halfwidth_advance: units_per_em / 2,
            line_gap: 0,
            column_gap: 0,
        }
    }

    /// Create standard 1000-unit metrics.
    pub fn standard() -> Self {
        Self::new(1000)
    }

    /// Set ascender.
    pub fn with_ascender(mut self, ascender: i64) -> Self {
        self.ascender = ascender;
        self
    }

    /// Set descender.
    pub fn with_descender(mut self, descender: i64) -> Self {
        self.descender = descender;
        self
    }

    /// Total line height.
    pub fn line_height(&self) -> i64 {
        self.ascender - self.descender + self.line_gap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_creation() {
        let font = CjkFont::new("TestFont")
            .with_script(Script::Han)
            .with_region(Region::JP);

        assert_eq!(font.name, "TestFont");
        assert_eq!(font.script, Script::Han);
        assert_eq!(font.region, Region::JP);
    }

    #[test]
    fn test_character_lookup() {
        let mut font = CjkFont::new("Test");
        font.add_character(Character::new(0x6C38)); // 永

        assert!(font.get_character(0x6C38).is_some());
        assert!(font.get_character_for_char('永').is_some());
        assert!(font.get_character(0x0041).is_none());
    }

    #[test]
    fn test_regional_variants() {
        let mut font = CjkFont::new("Test");

        // Add default character
        font.add_character(Character::new(0x76F4).with_region(Region::CN)); // 直

        // Add JP variant
        font.add_variant(
            Character::new(0x76F4).with_region(Region::JP),
            Region::JP,
        );

        let cn = font.get_character_for_region(0x76F4, Region::CN);
        let jp = font.get_character_for_region(0x76F4, Region::JP);

        assert!(cn.is_some());
        assert!(jp.is_some());
        assert_eq!(cn.unwrap().region, Some(Region::CN));
        assert_eq!(jp.unwrap().region, Some(Region::JP));
    }

    #[test]
    fn test_metrics() {
        let metrics = CjkFontMetrics::standard();
        assert_eq!(metrics.units_per_em, 1000);
        assert_eq!(metrics.fullwidth_advance, 1000);
        assert_eq!(metrics.halfwidth_advance, 500);
    }
}
