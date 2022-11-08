#![warn(clippy::invisible_characters)]
#![warn(clippy::non_ascii_literal)]
#![warn(clippy::unicode_not_nfc)]


/// This shows off some issues with Clippy's unicode lints vs raw strings
fn main() {
    let invis = r#"You don’t see it, but there may be a zero-width space or soft hyphen some­where in this text.

"#;

    let nonnfc = r#"You may not see it, but “à”" and “à”" aren’t the same string."#;

    // bonus
    println!(r#"Hello, world! — {} — {}"#, invis, nonnfc);
}
