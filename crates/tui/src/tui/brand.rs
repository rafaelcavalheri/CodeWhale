//! The launch-screen wordmark.
//!
//! This is presentation only. The public product name, the `CodeWhale`/`codew`
//! compatibility identifiers, protocol names, and storage keys are unchanged
//! and are not read from here — renaming any of those is a declared migration,
//! not a startup-screen edit. Nothing in this module reaches configuration,
//! telemetry, or the wire.
//!
//! The mark is authored glyph art, and every glyph it uses has a
//! [`crate::tui::glyphs::ascii_fallback`] entry, so the whole launch surface
//! still narrows under `CODEWHALE_ASCII_SAFE=1`.

/// Name shown on the startup screen.
pub const DISPLAY_NAME: &str = "Anarkodator";

/// Short form for terminals too narrow to spend eleven columns on a wordmark.
pub const DISPLAY_NAME_SHORT: &str = "ank";

/// Three-row block mark: an `A` with its crossbar, sitting on split feet.
pub const MARK: [&str; MARK_HEIGHT] = ["▗▛▀▜▖", "▐█▄█▌", "▝▘ ▝▘"];

pub const MARK_HEIGHT: usize = 3;

/// Display width of every [`MARK`] row. Asserted below rather than computed,
/// so an edit that unbalances the art fails the build's test pass instead of
/// silently ragging the three brand rows against each other.
pub const MARK_WIDTH: usize = 5;

/// Columns between the mark and the fact column beside it.
pub const MARK_GUTTER: usize = 2;

/// Total lead-in before the brand facts start.
pub const MARK_COLUMN: usize = MARK_WIDTH + MARK_GUTTER;

/// The glyph that opens the welcome card. `✦` has an ASCII fallback (`*`).
pub const CARD_MARK: &str = "✦";

#[cfg(test)]
mod tests {
    use super::*;
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn mark_rows_share_one_width() {
        for row in MARK {
            assert_eq!(row.width(), MARK_WIDTH, "unbalanced mark row {row:?}");
        }
    }

    #[test]
    fn mark_and_card_glyphs_narrow_to_ascii() {
        for row in MARK {
            for ch in row.chars().filter(|ch| !ch.is_ascii()) {
                assert!(
                    crate::tui::glyphs::ascii_fallback(&ch.to_string()).is_some(),
                    "mark glyph {ch:?} has no ASCII fallback"
                );
            }
        }
        assert!(crate::tui::glyphs::ascii_fallback(CARD_MARK).is_some());
    }
}
