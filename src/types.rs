//! Types and data structures for AksharaCore engine.

#![allow(dead_code)]

/// Represents a Unicode character or sequence in the Kannada Unicode block (U+0C80..U+0CFF).
pub const KANNADA_VIRAMA: char = '\u{0CCD}';   // ್ (Halant)
pub const KANNADA_ANUSVARA: char = '\u{0C82}'; // ಂ
pub const KANNADA_VISARGA: char = '\u{0C83}';  // ಃ
pub const KANNADA_ZWJ: char = '\u{200D}';      // Zero Width Joiner
pub const KANNADA_ZWNJ: char = '\u{200C}';     // Zero Width Non-Joiner

/// Categorization of Kannada phonetic elements in the KP Rao layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KannadaChar {
    /// Independent vowel, e.g., 'ಅ', 'ಆ', 'ಇ', 'ಈ', etc.
    IndependentVowel(char),
    /// Consonant with implicit 'a', e.g., 'ಕ', 'ಖ', 'ಗ', etc.
    Consonant(char),
    /// Dependent vowel sign (Gunithakshara / Matra), e.g., 'ಾ', 'ಿ', 'ೀ', etc.
    Matra(char),
    /// Yogavaahas (Anusvara 'ಂ', Visarga 'ಃ').
    Yogavaaha(char),
    /// Explicit Halant / Virama ('್') for creating explicit half-consonants or conjuncts (Ottu).
    Virama,
    /// Digits or symbols mapped directly.
    Direct(char),
}

/// The internal state of the transliteration finite state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineState {
    /// Resting state, ready for fresh input.
    Idle,
    /// A consonant was just emitted (has implicit 'a').
    ConsonantEmitted {
        consonant: char,
    },
    /// A Virama (್) was emitted (explicitly or via ottu trigger).
    ViramaEmitted {
        base_consonant: char,
    },
    /// A conjunct consonant was formed (e.g. ಕ + ್ + ಕ = ಕ್ಕ).
    ConjunctEmitted {
        last_consonant: char,
    },
}

impl Default for EngineState {
    fn default() -> Self {
        EngineState::Idle
    }
}

/// The resulting action instructing the Text Injector on what to do with the target window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyAction {
    /// Ignore this key or let it pass directly to the OS (e.g. navigation, modifiers).
    PassThrough,
    /// Suppress the physical key, do nothing.
    Suppress,
    /// Suppress the physical key and type the given string.
    Emit(String),
    /// Suppress the key, backspace N characters from target, and type new string.
    Replace {
        backspaces: usize,
        text: String,
    },
}
