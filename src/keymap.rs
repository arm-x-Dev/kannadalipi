//! KP Rao (KaGaPa) phonetic layout mapping tables for Nudi 4.0 muscle memory.
//!
//! Maps standard QWERTY keys (character + shift state) to pure modern Kannada Unicode.

use crate::types::KannadaChar;

/// Look up a Latin character (case-sensitive) to find its KP Rao phonetic classification.
pub fn lookup_kp_rao(ch: char) -> Option<KannadaChar> {
    match ch {
        // --- Vowels (Independent) ---
        // In KP Rao:
        // 'a' = ಅ, 'A' = ಆ
        // 'i' = ಇ, 'I' = ಈ
        // 'u' = ಉ, 'U' = ಊ
        // 'R' = ಋ
        // 'e' = ಎ, 'E' = ಏ
        // 'Y' = ಐ (ai)
        // 'o' = ಒ, 'O' = ಓ
        // 'V' = ಔ (au)
        'a' => Some(KannadaChar::IndependentVowel('\u{0C85}')), // ಅ
        'A' => Some(KannadaChar::IndependentVowel('\u{0C86}')), // ಆ
        'i' => Some(KannadaChar::IndependentVowel('\u{0C87}')), // ಇ
        'I' => Some(KannadaChar::IndependentVowel('\u{0C88}')), // ಈ
        'u' => Some(KannadaChar::IndependentVowel('\u{0C89}')), // ಉ
        'U' => Some(KannadaChar::IndependentVowel('\u{0C8A}')), // ಊ
        'R' => Some(KannadaChar::IndependentVowel('\u{0C8B}')), // ಋ
        'e' => Some(KannadaChar::IndependentVowel('\u{0C8E}')), // ಎ
        'E' => Some(KannadaChar::IndependentVowel('\u{0C8F}')), // ಏ
        'Y' => Some(KannadaChar::IndependentVowel('\u{0C90}')), // ಐ
        'o' => Some(KannadaChar::IndependentVowel('\u{0C92}')), // ಒ
        'O' => Some(KannadaChar::IndependentVowel('\u{0C93}')), // ಓ
        'V' => Some(KannadaChar::IndependentVowel('\u{0C94}')), // ಔ

        // --- Consonants (Vyanjanas) ---
        // Ka-varga
        'k' => Some(KannadaChar::Consonant('\u{0C95}')), // ಕ
        'K' => Some(KannadaChar::Consonant('\u{0C96}')), // ಖ
        'g' => Some(KannadaChar::Consonant('\u{0C97}')), // ಗ
        'G' => Some(KannadaChar::Consonant('\u{0C98}')), // ಘ
        'z' => Some(KannadaChar::Consonant('\u{0C99}')), // ಙ

        // Cha-varga
        'c' => Some(KannadaChar::Consonant('\u{0C9A}')), // ಚ
        'C' => Some(KannadaChar::Consonant('\u{0C9B}')), // ಛ
        'j' => Some(KannadaChar::Consonant('\u{0C9C}')), // ಜ
        'J' => Some(KannadaChar::Consonant('\u{0C9D}')), // ಝ
        'Z' => Some(KannadaChar::Consonant('\u{0C9E}')), // ಞ

        // Ta-varga (Retroflex)
        't' => Some(KannadaChar::Consonant('\u{0C9F}')), // ಟ
        'T' => Some(KannadaChar::Consonant('\u{0CA0}')), // ಠ
        'd' => Some(KannadaChar::Consonant('\u{0CA1}')), // ಡ
        'D' => Some(KannadaChar::Consonant('\u{0CA2}')), // ಢ
        'N' => Some(KannadaChar::Consonant('\u{0CA3}')), // ಣ

        // Tha-varga (Dental)
        'w' => Some(KannadaChar::Consonant('\u{0CA4}')), // ತ
        'W' => Some(KannadaChar::Consonant('\u{0CA5}')), // ಥ
        'x' => Some(KannadaChar::Consonant('\u{0CA6}')), // ದ
        'X' => Some(KannadaChar::Consonant('\u{0CA7}')), // ಧ
        'n' => Some(KannadaChar::Consonant('\u{0CA8}')), // ನ

        // Pa-varga
        'p' => Some(KannadaChar::Consonant('\u{0CAA}')), // ಪ
        'P' => Some(KannadaChar::Consonant('\u{0CAB}')), // ಫ
        'b' => Some(KannadaChar::Consonant('\u{0CAC}')), // ಬ
        'B' => Some(KannadaChar::Consonant('\u{0CAD}')), // ಭ
        'm' => Some(KannadaChar::Consonant('\u{0CAE}')), // ಮ

        // Other Consonants
        'y' => Some(KannadaChar::Consonant('\u{0CAF}')), // ಯ
        'r' => Some(KannadaChar::Consonant('\u{0CB0}')), // ರ
        'l' => Some(KannadaChar::Consonant('\u{0CB2}')), // ಲ
        'v' => Some(KannadaChar::Consonant('\u{0CB5}')), // ವ
        'S' => Some(KannadaChar::Consonant('\u{0CB6}')), // ಶ
        's' => Some(KannadaChar::Consonant('\u{0CB7}')), // ಷ
        'h' => Some(KannadaChar::Consonant('\u{0CB8}')), // ಸ
        'H' => Some(KannadaChar::Consonant('\u{0CB9}')), // ಹ
        'L' => Some(KannadaChar::Consonant('\u{0CB3}')), // ಳ (standard modern Kannada La)
        
        // --- Special Characters & Modifiers in KP Rao ---
        // 'f' or 'F' is the Halant/Virama / Ottu modifier key (್)
        'f' => Some(KannadaChar::Virama), // ್
        'F' => Some(KannadaChar::Virama), // ್ (Shift+F can also generate virama or explicit halant)
        
        // Yogavaahas
        // 'M' = Anusvara (ಂ)
        // ':' = Visarga (ಃ)
        'M' => Some(KannadaChar::Yogavaaha('\u{0C82}')), // ಂ
        ':' => Some(KannadaChar::Yogavaaha('\u{0C83}')), // ಃ

        // Kannada Digits (accessible via specific keys or numpad)
        // If mapped to digits with alt, or standard keys:
        _ => None,
    }
}

/// Helper: Get the dependent vowel sign (Matra) corresponding to a vowel key.
/// When typed after a consonant, KP Rao attaches this vowel sign.
pub fn vowel_to_matra(ch: char) -> Option<char> {
    match ch {
        'a' => None,                                   // 'a' gives back the implicit 'a' (no matra, removes virama if any)
        'A' => Some('\u{0CBE}'),                      // ಾ (Dīrgha)
        'i' => Some('\u{0CBF}'),                      // ಿ (Hrasva Gudi)
        'I' => Some('\u{0CC0}'),                      // ೀ (Dīrgha Gudi)
        'u' => Some('\u{0CC1}'),                      // ು (Kommu)
        'U' => Some('\u{0CC2}'),                      // ೂ (Dīrgha Kommu)
        'R' => Some('\u{0CC3}'),                      // ೃ (Vatrusuli)
        'e' => Some('\u{0CC6}'),                      // ೆ (Hrasva Ettu)
        'E' => Some('\u{0CC7}'),                      // ೇ (Dīrgha Ettu)
        'Y' => Some('\u{0CC8}'),                      // ೈ (Aitvam)
        'o' => Some('\u{0CCA}'),                      // ೊ (Ottu / Ottu-e)
        'O' => Some('\u{0CCB}'),                      // ೋ (Dīrgha Ottu)
        'V' => Some('\u{0CCC}'),                      // ೌ (Autvam)
        _ => None,
    }
}
