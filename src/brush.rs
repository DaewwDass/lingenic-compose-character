//! Brush definitions for character fonts.
//!
//! Named brush definitions, parallel to lettering's PenDef.

use brush_path::{Brush, BristleType};

/// Brush definition (named brush in font).
///
/// Wraps brush-path's Brush with a name and optional color.
#[derive(Debug, Clone)]
pub struct BrushDef {
    /// Brush name (for reference in strokes).
    pub name: String,

    /// The brush parameters.
    pub brush: Brush,

    /// Optional default ink name.
    pub ink: Option<String>,
}

impl BrushDef {
    /// Create a new brush definition.
    pub fn new(name: &str, brush: Brush) -> Self {
        Self {
            name: name.to_string(),
            brush,
            ink: None,
        }
    }

    /// Create a small brush (for dots, thin strokes).
    pub fn small(name: &str) -> Self {
        Self::new(name, Brush::small())
    }

    /// Create a medium brush (general purpose).
    pub fn medium(name: &str) -> Self {
        Self::new(name, Brush::medium())
    }

    /// Create a large brush (for bold strokes).
    pub fn large(name: &str) -> Self {
        Self::new(name, Brush::large())
    }

    /// Create a brush with specific dimensions.
    pub fn with_size(name: &str, tip_width: i64, belly_width: i64) -> Self {
        Self::new(
            name,
            Brush::new()
                .tip_width(tip_width)
                .belly_width(belly_width),
        )
    }

    /// Set bristle type.
    pub fn bristle(mut self, bristle: BristleType) -> Self {
        self.brush = self.brush.bristle(bristle);
        self
    }

    /// Set flexibility.
    pub fn flexibility(mut self, flex: f64) -> Self {
        self.brush = self.brush.flexibility(flex);
        self
    }

    /// Set default ink name.
    pub fn with_ink(mut self, ink: &str) -> Self {
        self.ink = Some(ink.to_string());
        self
    }

    /// Get the underlying brush.
    pub fn brush(&self) -> &Brush {
        &self.brush
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brush_def_creation() {
        let brush = BrushDef::medium("main");
        assert_eq!(brush.name, "main");
    }

    #[test]
    fn test_brush_def_with_bristle() {
        let brush = BrushDef::medium("goat")
            .bristle(BristleType::Goat)
            .flexibility(0.8);
        assert_eq!(brush.brush.bristle, BristleType::Goat);
    }

    #[test]
    fn test_brush_def_sizes() {
        let small = BrushDef::small("s");
        let large = BrushDef::large("l");
        assert!(small.brush.tip_width < large.brush.tip_width);
    }
}
