//! Regional variants for CJK characters.
//!
//! The same Unicode codepoint may render differently across regions.
//! For example, 直 has subtle differences in CN, TW, JP, KR forms.

/// Regional variant for CJK characters.
///
/// Same character may have different standard forms across regions.
/// Unicode provides variation selectors, but fonts often need to
/// choose a default and provide alternates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Region {
    /// Simplified Chinese (PRC standard, GB).
    #[default]
    CN,

    /// Traditional Chinese (Taiwan standard, Big5/CNS).
    TW,

    /// Traditional Chinese (Hong Kong standard, HKSCS).
    HK,

    /// Japanese (JIS standard).
    JP,

    /// Korean (KS standard).
    KR,

    /// Vietnamese (Chữ Nôm).
    VN,
}

impl Region {
    /// ISO 15924 script code suffix used in font naming.
    pub fn script_tag(&self) -> &'static str {
        match self {
            Region::CN => "Hans",
            Region::TW => "Hant",
            Region::HK => "Hant-HK",
            Region::JP => "Jpan",
            Region::KR => "Kore",
            Region::VN => "Hani",
        }
    }

    /// Language tag for this region.
    pub fn lang_tag(&self) -> &'static str {
        match self {
            Region::CN => "zh-CN",
            Region::TW => "zh-TW",
            Region::HK => "zh-HK",
            Region::JP => "ja",
            Region::KR => "ko",
            Region::VN => "vi",
        }
    }

    /// Whether this region uses simplified characters.
    pub fn is_simplified(&self) -> bool {
        matches!(self, Region::CN)
    }

    /// Whether this region uses traditional characters.
    pub fn is_traditional(&self) -> bool {
        matches!(self, Region::TW | Region::HK)
    }

    /// Regions that share the same Unicode codepoints but may differ in glyph form.
    pub fn han_unified_regions() -> &'static [Region] {
        &[Region::CN, Region::TW, Region::HK, Region::JP, Region::KR, Region::VN]
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lang_tag())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_tags() {
        assert_eq!(Region::CN.script_tag(), "Hans");
        assert_eq!(Region::JP.script_tag(), "Jpan");
        assert_eq!(Region::CN.lang_tag(), "zh-CN");
    }

    #[test]
    fn test_simplified_traditional() {
        assert!(Region::CN.is_simplified());
        assert!(!Region::CN.is_traditional());
        assert!(Region::TW.is_traditional());
        assert!(!Region::JP.is_simplified());
    }
}
