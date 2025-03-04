#[macro_use]
mod generated;
mod syntax_node;

pub use self::generated::*;
pub use biome_rowan::{
    SyntaxNodeText, TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent,
};
pub use syntax_node::*;

use crate::CssSyntaxKind::*;
use biome_rowan::{AstNode, RawSyntaxKind};

impl From<u16> for CssSyntaxKind {
    fn from(d: u16) -> CssSyntaxKind {
        assert!(d <= (CssSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, CssSyntaxKind>(d) }
    }
}

impl From<CssSyntaxKind> for u16 {
    fn from(k: CssSyntaxKind) -> u16 {
        k as u16
    }
}

impl CssSyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            CssSyntaxKind::NEWLINE | CssSyntaxKind::WHITESPACE | CssSyntaxKind::COMMENT
        )
    }

    /// Returns `true` for any contextual (await) or non-contextual keyword
    #[inline]
    pub const fn is_keyword(self) -> bool {
        true
    }

    /// Returns `true` for contextual keywords
    #[inline]
    pub const fn is_contextual_keyword(self) -> bool {
        (self as u16) >= (ALICEBLUE_KW as u16) && (self as u16) <= (VAR_KW as u16)
    }

    /// Returns `true` for contextual attribute modifier keywords
    #[inline]
    pub const fn is_attribute_modifier_keyword(self) -> bool {
        let k = self as u16;
        k == (I_KW as u16) || k == (S_KW as u16)
    }

    /// Returns true for all non-contextual keywords (includes future reserved keywords)
    #[inline]
    pub const fn is_non_contextual_keyword(self) -> bool {
        self.is_keyword() && !self.is_contextual_keyword()
    }
}

impl biome_rowan::SyntaxKind for CssSyntaxKind {
    const TOMBSTONE: Self = CssSyntaxKind::TOMBSTONE;
    const EOF: Self = EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            CSS_BOGUS
                | CSS_BOGUS_RULE
                | CSS_BOGUS_SELECTOR
                | CSS_BOGUS_SUB_SELECTOR
                | CSS_BOGUS_BODY
                | CSS_BOGUS_PSEUDO_CLASS
        )
    }

    fn to_bogus(&self) -> Self {
        match self {
            kind if AnyCssSubSelector::can_cast(*kind) => CSS_BOGUS_SUB_SELECTOR,
            kind if AnyCssSelector::can_cast(*kind) => CSS_BOGUS_SELECTOR,
            kind if AnyCssRule::can_cast(*kind) => CSS_BOGUS_RULE,
            kind if AnyCssPseudoClass::can_cast(*kind) => CSS_BOGUS_PSEUDO_CLASS,

            _ => CSS_BOGUS,
        }
    }

    #[inline]
    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    #[inline]
    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        matches!(self, CSS_ROOT)
    }

    #[inline]
    fn is_list(&self) -> bool {
        CssSyntaxKind::is_list(*self)
    }

    fn to_string(&self) -> Option<&'static str> {
        CssSyntaxKind::to_string(self)
    }
}

impl TryFrom<CssSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: CssSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                CssSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
                CssSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
                CssSyntaxKind::COMMENT => Ok(TriviaPieceKind::SingleLineComment),
                CssSyntaxKind::MULTILINE_COMMENT => Ok(TriviaPieceKind::MultiLineComment),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}
