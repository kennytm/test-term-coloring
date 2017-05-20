extern crate term;

use std::io;
use term::Terminal;
use term::terminfo::TermInfo;

fn check<T: Terminal>(name: &str, result: Option<T>) -> io::Result<()> {
    match result {
        Some(mut t) => {
            println!("{}::supports_color() = {}", name, t.supports_color());
            t.fg(term::color::GREEN)?;
            writeln!(t, "this text should be green")?;
            t.reset()?;
        }
        None => {
            println!("Failed to create a {}", name);
        }
    }
    Ok(())
}


fn main() {
    println!("\x1b[31;1m~~ Yes color is supported in AppVeyor ~~\x1b[0m");

    println!("{:?}", TermInfo::from_env().is_ok());

    // check("WinConsole", term::WinConsole::new(io::stdout()).ok()).unwrap();
    check("TerminfoTerminal", term::TerminfoTerminal::new(io::stdout())).unwrap();
}