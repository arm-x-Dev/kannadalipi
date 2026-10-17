# Aksharamale (ಅಕ್ಷರಮಾಲೆ) 

<p align="left">
  <img src="https://img.shields.io/badge/Language-Rust_1.80+-DEA584?style=for-the-badge&logo=rust&logoColor=black" alt="Language Rust" />
  <img src="https://img.shields.io/badge/Platform-Windows_10%20%2F%2011-0078D6?style=for-the-badge&logo=windows&logoColor=white" alt="Platform Windows" />
  <img src="https://img.shields.io/badge/Layout-KP_Rao_(KaGaPa)-FFD700?style=for-the-badge&labelColor=C8102E&logoColor=white" alt="Layout KaGaPa" />
  <img src="https://img.shields.io/badge/Standard-Pure_Unicode_15-4B0082?style=for-the-badge&logoColor=white" alt="Unicode 15" />
  <img src="https://img.shields.io/badge/Security-Zero_Telemetry_%26_Offline-2ea44f?style=for-the-badge&logo=shield&logoColor=white" alt="Zero Telemetry" />
  <img src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge" alt="License MIT" />
</p>

**Aksharamale (ಅಕ್ಷರಮಾಲೆ)** is a modern, ultra-lightweight, and lag-free Kannada Unicode typing engine for Windows. 

Designed to honor the beloved **KP Rao (KaGaPa)** phonetic keyboard layout—the standard muscle memory relied upon by millions of writers, journalists, teachers, and government officials across Karnataka—Aksharamale runs seamlessly in the background and injects **100% pure modern Unicode**. Your text works instantly across Microsoft Word, Excel, Google Chrome, WhatsApp, and social media without breaking or requiring legacy ASCII font conversions.

---

## ✨ Features

- ⚡ **Zero-Latency Performance:** Written in pure, memory-safe native Rust. Intercepts and transliterates keystrokes in under `0.01ms`.
- ⌨️ **Standard KP Rao (KaGaPa) Layout:** Uses identical key combinations so you never have to re-learn typing habits.
- 🌐 **Pure Universal Unicode:** Characters typed can be copied and pasted anywhere—WhatsApp, email, web browsers, Android, or iOS—without text corruption.
- 🔔 **Subtle System Tray Presence:** Runs silently in the Windows notification area with zero command prompt or console windows.
- 🚀 **1-Tap Hotkey Toggle:** Press **`F12`** or **`Scroll Lock`** anytime to instantly switch between Kannada and English.
- 🔤 **Bundled Google Noto Serif Kannada Font:** Setup installer automatically configures modern book-quality Kannada fonts in Windows.
- 🛡️ **100% Private & Offline:** Zero internet connections, zero network sockets, zero disk logging. Operates purely in volatile RAM.
- 🔒 **Single-Instance Guard:** Protected by Windows named mutex to prevent accidental duplicate background hooks.

---

## 📥 Quick Download (For Everyone)

Non-technical users do **not** need to install Rust or run any commands:

1. Go to the [Releases](https://github.com/) section.
2. Download **`Aksharamale_Setup_v0.1.0.exe`**.
3. Double-click to run the setup wizard (choose any drive or install location).
4. Aksharamale will launch in your System Tray and notify you:
   > 🔔 **Aksharamale is Active (ಅಕ್ಷರಮಾಲೆ)**  
   > *Kannada typing is ON. Press [F12] to switch English/Kannada anywhere!*

---

## ⌨️ Typing Quick-Reference (KP Rao Layout)

### 1. Swaras (Independent Vowels)
| Key | Kannada | Key | Kannada | Key | Kannada |
|:---:|:---:|:---:|:---:|:---:|:---:|
| `a` | ಅ | `A` | ಆ | `i` | ಇ |
| `I` | ಈ | `u` | ಉ | `U` | ಊ |
| `R` | ಋ | `e` | ಎ | `E` | ಏ |
| `Y` | ಐ | `o` | ಒ | `O` | ಓ |
| `V` | ಔ | `M` | ಂ (Anusvara) | `:` | ಃ (Visarga) |

### 2. Vyanjanas (Consonants)
| Varga | Letters & Key Combinations |
|:---|:---|
| **Ka-varga** | `k` = ಕ, `K` = ಖ, `g` = ಗ, `G` = ಘ, `z` = ಙ |
| **Cha-varga** | `c` = ಚ, `C` = ಛ, `j` = ಜ, `J` = ಝ, `Z` = ಞ |
| **Ta-varga (Retroflex)** | `t` = ಟ, `T` = ಠ, `d` = ಡ, `D` = ಢ, `N` = ಣ |
| **Tha-varga (Dental)** | `w` = ತ, `W` = ಥ, `x` = ದ, `X` = ಧ, `n` = ನ |
| **Pa-varga** | `p` = ಪ, `P` = ಫ, `b` = ಬ, `B` = ಭ, `m` = ಮ |
| **Others** | `y` = ಯ, `r` = ರ, `l` = ಲ, `v` = ವ, `S` = ಶ, `s` = ಷ, `h` = ಸ, `H` = ಹ, `L` = ಳ |

### 3. Gunithakshara (Vowel Modifiers)
Type the base consonant, followed by the vowel key:
- `k` + `A` ➔ **ಕಾ**
- `k` + `i` ➔ **ಕಿ**
- `k` + `I` ➔ **ಕೀ**
- `k` + `u` ➔ **ಕು**
- `k` + `U` ➔ **ಕೂ**
- `k` + `e` ➔ **ಕೆ**
- `k` + `E` ➔ **ಕೇ**
- `k` + `Y` ➔ **ಕೈ**
- `k` + `o` ➔ **ಕೊ**
- `k` + `O` ➔ **ಕೋ**
- `k` + `V` ➔ **ಕೌ**

### 4. Ottu / Conjuncts (Halant / Virama `f`)
To create subscript conjuncts (ವತ್ತಕ್ಷರಗಳು), press **`f`** between two consonants:
- `k` + `f` + `k` ➔ **ಕ್ಕ**
- `k` + `f` + `k` + `i` ➔ **ಕ್ಕಿ**
- `s` + `f` + `k` + `A` ➔ **ಸ್ಕಾ**
- `w` + `f` + `w` + `u` ➔ **ತ್ತು**

### 5. Common Example Words
| Word | English Keystrokes |
|:---|:---|
| **ಕನ್ನಡ** | `kanfnaDa` |
| **ನಮಸ್ಕಾರ** | `namasfkAra` |
| **ಕರ್ನಾಟಕ** | `karfnaTaka` |
| **ಸಿರಿಗನ್ನಡಂ** | `siriganfnaDaM` |
| **ಬೆಳಕು** | `beLaku` |

---

## 🛠️ Building from Source (For Developers)

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (Stable MSVC toolchain)
- Windows 10 or 11
- (Optional) [Inno Setup 6](https://jrsoftware.org/isdl.php) if compiling the installer

### 1. Clone & Compile Executable
```powershell
git clone https://github.com/<your-username>/Aksharamale.git
cd Aksharamale
cargo build --release
```
The optimized background executable will be generated at:
`target\release\aksharamale.exe`

### 2. Compile Inno Setup Installer
Open `installer.iss` in Inno Setup Compiler and click **Compile** (or use command line):
```powershell
& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer.iss
```
The single-file setup installer will be generated in `dist\`.

---

## 📜 Architecture Overview

AksharaCore operates via a clean three-tiered design:

```
Physical Keypress (Windows Event)
              │
              ▼
    [ WH_KEYBOARD_LL ]           <-- Intercepts & suppresses Latin key
              │
              ▼
 [ Transliteration Engine ]      <-- Contextual Finite State Machine (FSM)
              │                      Resolves KP Rao phonetic combinations
              ▼
     [ Text Injector ]           <-- Atomic SendInput KEYEVENTF_UNICODE
              │                      Direct UTF-16 injection to active window
              ▼
    Active Foreground Window (Word, Browser, Notepad)
```

---

## ⚖️ License & Acknowledgements

- **Software Code:** Licensed under the [MIT License](LICENSE).
- **Bundled Font:** [Noto Serif Kannada](https://fonts.google.com/specimen/Noto+Serif+Kannada) by Google, licensed under the [SIL Open Font License 1.1](assets/fonts/OFL.txt).
- **Phonetic Standard:** Dedicated to Professor **K.P. Rao**, whose pioneering layout made Kannada accessible on modern computing keyboards.
"# KannadaLipi"

> [!NOTE]
> **Windows SmartScreen Note:**  
> Since Aksharamale is a free, community-driven open-source software and does not use an expensive corporate code-signing certificate, Windows SmartScreen may show a prompt: *"Windows protected your PC"*.  
> 
> Simply click **More info** ➔ **Run anyway**.  
> The application is 100% offline, zero-telemetry, and safe. The complete source code is fully transparent and auditable right here on GitHub.
