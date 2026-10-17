//! Transliteration Finite State Machine (FSM) for AksharaCore.
//! Handles contextual phonetic composition: consonants, matras, virama, and conjuncts (ottu).

#![allow(dead_code)]

use crate::keymap::{lookup_kp_rao, vowel_to_matra};
use crate::types::{EngineState, KannadaChar, KeyAction, KANNADA_VIRAMA};

pub struct TransliterationEngine {
    state: EngineState,
}

impl TransliterationEngine {
    pub fn new() -> Self {
        Self {
            state: EngineState::Idle,
        }
    }

    /// Reset engine state back to Idle (e.g. on Space, Enter, Escape, Arrow keys).
    pub fn reset(&mut self) {
        self.state = EngineState::Idle;
    }

    /// Get current state (useful for inspecting or testing).
    pub fn current_state(&self) -> &EngineState {
        &self.state
    }

    /// Process a typed character from keyboard input.
    pub fn process_char(&mut self, ch: char) -> KeyAction {
        // If it's whitespace or punctuation that breaks akshara context, pass through and reset state
        if ch.is_whitespace() || ch == '\r' || ch == '\n' {
            self.reset();
            return KeyAction::PassThrough;
        }

        // Look up the character in the KP Rao mapping table
        let parsed_char = match lookup_kp_rao(ch) {
            Some(k) => k,
            None => {
                self.reset();
                return KeyAction::PassThrough;
            }
        };

        match (&self.state, parsed_char) {
            // --- IDLE STATE ---
            (EngineState::Idle, KannadaChar::IndependentVowel(v)) => {
                self.state = EngineState::Idle;
                KeyAction::Emit(v.to_string())
            }
            (EngineState::Idle, KannadaChar::Consonant(c)) => {
                self.state = EngineState::ConsonantEmitted { consonant: c };
                KeyAction::Emit(c.to_string())
            }
            (EngineState::Idle, KannadaChar::Virama) => {
                self.state = EngineState::ViramaEmitted { base_consonant: '\0' };
                KeyAction::Emit(KANNADA_VIRAMA.to_string())
            }
            (EngineState::Idle, KannadaChar::Yogavaaha(y)) => {
                self.state = EngineState::Idle;
                KeyAction::Emit(y.to_string())
            }
            (EngineState::Idle, KannadaChar::Direct(d)) => {
                self.state = EngineState::Idle;
                KeyAction::Emit(d.to_string())
            }

            // --- CONSONANT EMITTED STATE ---
            (EngineState::ConsonantEmitted { consonant: _ }, KannadaChar::IndependentVowel(_)) => {
                if let Some(matra) = vowel_to_matra(ch) {
                    self.state = EngineState::Idle;
                    KeyAction::Emit(matra.to_string())
                } else if ch == 'a' {
                    self.state = EngineState::Idle;
                    KeyAction::Suppress
                } else {
                    self.state = EngineState::Idle;
                    KeyAction::PassThrough
                }
            }

            (EngineState::ConsonantEmitted { consonant }, KannadaChar::Virama) => {
                let base = *consonant;
                self.state = EngineState::ViramaEmitted { base_consonant: base };
                KeyAction::Emit(KANNADA_VIRAMA.to_string())
            }

            (EngineState::ConsonantEmitted { .. }, KannadaChar::Consonant(new_c)) => {
                self.state = EngineState::ConsonantEmitted { consonant: new_c };
                KeyAction::Emit(new_c.to_string())
            }

            (EngineState::ConsonantEmitted { .. }, KannadaChar::Yogavaaha(y)) => {
                self.state = EngineState::Idle;
                KeyAction::Emit(y.to_string())
            }

            // --- VIRAMA EMITTED STATE ---
            (EngineState::ViramaEmitted { base_consonant: _ }, KannadaChar::Consonant(conjunct_c)) => {
                self.state = EngineState::ConjunctEmitted { last_consonant: conjunct_c };
                KeyAction::Emit(conjunct_c.to_string())
            }

            (EngineState::ViramaEmitted { .. }, _) => {
                self.reset();
                KeyAction::PassThrough
            }

            // --- CONJUNCT EMITTED STATE ---
            (EngineState::ConjunctEmitted { last_consonant: _ }, KannadaChar::IndependentVowel(_)) => {
                if let Some(matra) = vowel_to_matra(ch) {
                    self.state = EngineState::Idle;
                    KeyAction::Emit(matra.to_string())
                } else if ch == 'a' {
                    self.state = EngineState::Idle;
                    KeyAction::Suppress
                } else {
                    self.state = EngineState::Idle;
                    KeyAction::PassThrough
                }
            }

            (EngineState::ConjunctEmitted { last_consonant }, KannadaChar::Virama) => {
                let last = *last_consonant;
                self.state = EngineState::ViramaEmitted { base_consonant: last };
                KeyAction::Emit(KANNADA_VIRAMA.to_string())
            }

            (EngineState::ConjunctEmitted { .. }, KannadaChar::Consonant(c)) => {
                self.state = EngineState::ConsonantEmitted { consonant: c };
                KeyAction::Emit(c.to_string())
            }

            (EngineState::ConjunctEmitted { .. }, KannadaChar::Yogavaaha(y)) => {
                self.state = EngineState::Idle;
                KeyAction::Emit(y.to_string())
            }

            _ => {
                self.reset();
                KeyAction::PassThrough
            }
        }
    }
}
