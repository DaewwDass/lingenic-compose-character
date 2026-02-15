//! Script types for CJK writing systems.
//!
//! Different scripts have different structural characteristics
//! and may use different primitives (pen-path vs brush-path).

/// Script type within CJK writing systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Script {
    /// Han characters (漢字/汉字/漢字/한자).
    /// Used in Chinese, Japanese (kanji), Korean (hanja), Vietnamese (Chữ Nôm).
    #[default]
    Han,

    /// Japanese Hiragana (ひらがな).
    /// Cursive syllabary derived from Chinese characters.
    Hiragana,

    /// Japanese Katakana (カタカナ).
    /// Angular syllabary derived from Chinese characters.
    Katakana,

    /// Korean Hangul (한글).
    /// Alphabetic blocks composed of jamo.
    Hangul,

    /// Chinese Bopomofo (ㄅㄆㄇㄈ).
    /// Phonetic symbols used in Taiwan.
    Bopomofo,
}

impl Script {
    /// ISO 15924 script code.
    pub fn code(&self) -> &'static str {
        match self {
            Script::Han => "Hani",
            Script::Hiragana => "Hira",
            Script::Katakana => "Kana",
            Script::Hangul => "Hang",
            Script::Bopomofo => "Bopo",
        }
    }

    /// English name of the script.
    pub fn name(&self) -> &'static str {
        match self {
            Script::Han => "Han",
            Script::Hiragana => "Hiragana",
            Script::Katakana => "Katakana",
            Script::Hangul => "Hangul",
            Script::Bopomofo => "Bopomofo",
        }
    }

    /// Native name of the script.
    pub fn native_name(&self) -> &'static str {
        match self {
            Script::Han => "漢字",
            Script::Hiragana => "ひらがな",
            Script::Katakana => "カタカナ",
            Script::Hangul => "한글",
            Script::Bopomofo => "注音符號",
        }
    }

    /// Whether this script is logographic (meaning-based).
    pub fn is_logographic(&self) -> bool {
        matches!(self, Script::Han)
    }

    /// Whether this script is syllabic.
    pub fn is_syllabic(&self) -> bool {
        matches!(self, Script::Hiragana | Script::Katakana | Script::Bopomofo)
    }

    /// Whether this script is alphabetic.
    pub fn is_alphabetic(&self) -> bool {
        matches!(self, Script::Hangul)
    }

    /// Whether this script originated from brush calligraphy.
    pub fn is_brush_origin(&self) -> bool {
        // All CJK scripts derive from brush writing
        // Even Hangul, though designed, was written with brush historically
        true
    }

    /// Whether brush-path primitives are preferred for this script.
    ///
    /// Calligraphic styles use brush-path.
    /// Typographic styles (Song/Hei) may prefer pen-path.
    pub fn prefers_brush_path(&self) -> bool {
        // Script-level preference; actual choice depends on font style
        matches!(self, Script::Han | Script::Hiragana | Script::Katakana)
    }
}

impl std::fmt::Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_codes() {
        assert_eq!(Script::Han.code(), "Hani");
        assert_eq!(Script::Hiragana.code(), "Hira");
        assert_eq!(Script::Hangul.code(), "Hang");
    }

    #[test]
    fn test_script_types() {
        assert!(Script::Han.is_logographic());
        assert!(Script::Hiragana.is_syllabic());
        assert!(Script::Hangul.is_alphabetic());
    }

    #[test]
    fn test_native_names() {
        assert_eq!(Script::Han.native_name(), "漢字");
        assert_eq!(Script::Hangul.native_name(), "한글");
    }
}
