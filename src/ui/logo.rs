// VyCode - ASCII Logo
#![allow(dead_code)]

pub const LOGO: &str = r#"
 ██╗   ██╗██╗   ██╗ ██████╗ ██████╗ ██████╗ ███████╗
 ██║   ██║╚██╗ ██╔╝██╔════╝██╔═══██╗██╔══██╗██╔════╝
 ██║   ██║ ╚████╔╝ ██║     ██║   ██║██║  ██║█████╗  
 ╚██╗ ██╔╝  ╚██╔╝  ██║     ██║   ██║██║  ██║██╔══╝  
  ╚████╔╝    ██║   ╚██████╗╚██████╔╝██████╔╝███████╗
   ╚═══╝     ╚═╝    ╚═════╝ ╚═════╝ ╚═════╝ ╚══════╝"#;

pub const TAGLINE: &str = "AI Coding Terminal Assistant";
pub const AUTHOR: &str = "Created by Muhammad Lutfi Muzaki";
pub const GITHUB: &str = "https://github.com/MuhammadLutfiMuzakiiVY";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const LOGO_SMALL: &str = r#"
 ╦  ╦╦ ╦╔═╗╔═╗╔╦╗╔═╗
 ╚╗╔╝╚╦╝║  ║ ║ ║║║╣ 
  ╚╝  ╩ ╚═╝╚═╝═╩╝╚═╝"#;

pub const DIVIDER: &str = "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━";

pub fn startup_lines() -> Vec<&'static str> {
    vec![
        "",
        TAGLINE,
        "",
        AUTHOR,
        GITHUB,
        "",
        &DIVIDER,
        "",
        "Press any key to continue...",
    ]
}
