pub const QUIT_INSTRUCTIONS: &str = "Press F10 or q/Q to quit.";

const BANNER: &str = r#"
██████╗ ██╗   ██╗███████╗████████╗ ██████╗ ██████╗ 
██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗
██████╔╝██║   ██║║██████╗   ██║   ██║   ██║██████╔╝
██╔══██╗██║   ██║╚════██║   ██║   ██║   ██║██╔═══╝
██║  ██║╚██████╔╝██████╔╝   ██║   ╚██████╔╝██║  
╚═╝  ╚═╝ ╚═════╝ ╚═════╝    ╚═╝    ╚═════╝ ╚═╝  

          Rust System & GPU Monitor (rustop)
          Press F10 or q/Q to quit.
"#;

#[allow(dead_code)]
pub fn print_banner() {
    println!("{}", BANNER);
}

pub fn banner_text() -> &'static str {
    BANNER
}

pub fn make_bar(pct: f32) -> String {
    let ratio = pct / 100.0;
    let total_blocks = 20usize;

    let filled = (ratio * total_blocks as f32).round() as usize;
    let empty = total_blocks - filled;

    format!("[{}{}] {:5.1}%", "█".repeat(filled), "-".repeat(empty), pct)
}
