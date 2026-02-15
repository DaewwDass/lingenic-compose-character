//! Character strokes — the building blocks of CJK characters.
//!
//! Strokes can use either pen-path (for typographic styles like 宋體/黑體)
//! or brush-path (for calligraphic styles like 楷書/行書) primitives.

use brush_path::{
    BrushStroke, StrokeKind, EntryStyle, ExitStyle, ConnectionStyle,
};
use pen_path::Path;

/// A stroke within a character.
///
/// Can be either a brush stroke (calligraphic) or a pen stroke (typographic).
#[derive(Debug, Clone)]
pub enum CharacterStroke {
    /// Brush-based stroke (楷書, 行書, 草書 styles).
    /// Uses brush-path primitives with full semantics.
    Brush(BrushStrokeData),

    /// Pen-based stroke (宋體, 黑體 styles).
    /// Uses pen-path primitives, similar to Western fonts.
    Pen(PenStrokeData),
}

/// Brush stroke data — calligraphic stroke with full semantics.
#[derive(Debug, Clone)]
pub struct BrushStrokeData {
    /// The brush to use (by name).
    pub brush_name: String,

    /// Stroke classification (橫, 豎, 撇, etc.).
    pub kind: StrokeKind,

    /// The stroke path with dynamics and actions.
    pub stroke: BrushStroke,

    /// Optional ink override.
    pub ink_name: Option<String>,
}

/// Pen stroke data — typographic stroke like Western fonts.
#[derive(Debug, Clone)]
pub struct PenStrokeData {
    /// The pen to use (by name).
    pub pen_name: String,

    /// The path.
    pub path: Path,

    /// Optional width track (variable width along path).
    pub width_track: Option<Vec<i64>>,

    /// Optional color override.
    pub color: Option<String>,
}

impl CharacterStroke {
    /// Create a brush stroke.
    pub fn brush(brush_name: &str, kind: StrokeKind, stroke: BrushStroke) -> Self {
        CharacterStroke::Brush(BrushStrokeData {
            brush_name: brush_name.to_string(),
            kind,
            stroke,
            ink_name: None,
        })
    }

    /// Create a pen stroke.
    pub fn pen(pen_name: &str, path: Path) -> Self {
        CharacterStroke::Pen(PenStrokeData {
            pen_name: pen_name.to_string(),
            path,
            width_track: None,
            color: None,
        })
    }

    /// Check if this is a brush stroke.
    pub fn is_brush(&self) -> bool {
        matches!(self, CharacterStroke::Brush(_))
    }

    /// Check if this is a pen stroke.
    pub fn is_pen(&self) -> bool {
        matches!(self, CharacterStroke::Pen(_))
    }

    /// Get stroke kind (only for brush strokes).
    pub fn kind(&self) -> Option<StrokeKind> {
        match self {
            CharacterStroke::Brush(data) => Some(data.kind),
            CharacterStroke::Pen(_) => None,
        }
    }

    /// Get the brush name (only for brush strokes).
    pub fn brush_name(&self) -> Option<&str> {
        match self {
            CharacterStroke::Brush(data) => Some(&data.brush_name),
            CharacterStroke::Pen(_) => None,
        }
    }

    /// Get the pen name (only for pen strokes).
    pub fn pen_name(&self) -> Option<&str> {
        match self {
            CharacterStroke::Pen(data) => Some(&data.pen_name),
            CharacterStroke::Brush(_) => None,
        }
    }
}

impl BrushStrokeData {
    /// Create a new brush stroke data.
    pub fn new(brush_name: &str, kind: StrokeKind, stroke: BrushStroke) -> Self {
        Self {
            brush_name: brush_name.to_string(),
            kind,
            stroke,
            ink_name: None,
        }
    }

    /// Set ink override.
    pub fn with_ink(mut self, ink: &str) -> Self {
        self.ink_name = Some(ink.to_string());
        self
    }

    /// Get entry style (入鋒).
    pub fn entry(&self) -> EntryStyle {
        self.stroke.semantics.entry
    }

    /// Get exit style (收鋒).
    pub fn exit(&self) -> ExitStyle {
        self.stroke.semantics.exit
    }

    /// Get connection style (筆勢).
    pub fn connection(&self) -> ConnectionStyle {
        self.stroke.semantics.connection
    }

    /// Whether this stroke connects to the next.
    pub fn connects_to_next(&self) -> bool {
        self.stroke.semantics.connection.connects()
    }
}

impl PenStrokeData {
    /// Create a new pen stroke data.
    pub fn new(pen_name: &str, path: Path) -> Self {
        Self {
            pen_name: pen_name.to_string(),
            path,
            width_track: None,
            color: None,
        }
    }

    /// Set width track.
    pub fn with_width_track(mut self, track: Vec<i64>) -> Self {
        self.width_track = Some(track);
        self
    }

    /// Set color override.
    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use brush_path::Brush;

    #[test]
    fn test_brush_stroke() {
        let brush_stroke = BrushStroke::new(Brush::medium(), StrokeKind::Heng);
        let stroke = CharacterStroke::brush("main", StrokeKind::Heng, brush_stroke);

        assert!(stroke.is_brush());
        assert!(!stroke.is_pen());
        assert_eq!(stroke.kind(), Some(StrokeKind::Heng));
        assert_eq!(stroke.brush_name(), Some("main"));
    }

    #[test]
    fn test_pen_stroke() {
        let path = Path::new();
        let stroke = CharacterStroke::pen("mono", path);

        assert!(stroke.is_pen());
        assert!(!stroke.is_brush());
        assert_eq!(stroke.kind(), None);
        assert_eq!(stroke.pen_name(), Some("mono"));
    }
}
