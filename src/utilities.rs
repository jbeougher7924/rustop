pub const QUIT_INSTRUCTIONS: &str = "Press F10 or q/Q to quit.";

#[allow(dead_code)]
pub fn print_banner() {
    println!("{}", banner_text());
}

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
