//! Utilities for shared UI text and formatting helpers.

/// Shared quit instructions string used across the UI.
pub const QUIT_INSTRUCTIONS: &str = "Press F10 or q/Q to quit.";

#[allow(dead_code)]
/// Print the ASCII banner to stdout for non-TUI use.
pub fn print_banner() {
    println!("{}", banner_text());
}

/// Build the ASCII banner text displayed at the top of the TUI.
pub fn banner_text() -> String {
    format!(
        r#"
██████╗ ██╗   ██╗███████╗████████╗ ██████╗ ██████╗ 
██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗
██████╔╝██║   ██║║██████╗   ██║   ██║   ██║██████╔╝
██╔══██╗██║   ██║╚════██║   ██║   ██║   ██║██╔═══╝
██║  ██║╚██████╔╝██████╔╝   ██║   ╚██████╔╝██║  
╚═╝  ╚═╝ ╚═════╝ ╚═════╝    ╚═╝    ╚═════╝ ╚═╝  

          Rust System & GPU Monitor (rustop)        
"#,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_contains_title() {
        let banner = banner_text();
        assert!(
            banner.contains("Rust System & GPU Monitor (rustop)"),
            "banner should include product name"
        );
    }

    #[test]
    fn quit_instructions_are_present() {
        // The banner intentionally omits quit text; ensure the shared constant still matches expectation.
        assert_eq!(QUIT_INSTRUCTIONS, "Press F10 or q/Q to quit.");
    }
}
