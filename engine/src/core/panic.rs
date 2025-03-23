use std::fmt::Write as FmtWrite;
use std::mem;

use backtrace::Backtrace;
use parking_lot::Once;
use regex::Regex;

static INIT: parking_lot::Once = Once::new();

pub fn set_panic_hook() {
    use std::io::Write;

    use colored::Colorize;

    use crate::workspace;

    INIT.call_once(|| {
        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let log_path = workspace::get_working_dir().unwrap_or_else(|_| {
                default_hook(info);
                std::process::exit(0);
            });
            if !log_path.exists() {
                std::fs::create_dir_all(&log_path).unwrap_or_else(|_| {
                    default_hook(info);
                    std::process::exit(0);
                });
            }
            let log_path = log_path.join("panic.log");

            let mut file = std::fs::File::create(&log_path).unwrap_or_else(|_| {
                default_hook(info);
                std::process::exit(0);
            });
            
            // Instead of using payload_as_str(), downcast the panic payload:
            let payload = info.payload();
            let payload_str = if let Some(s) = payload.downcast_ref::<&str>() {
                *s
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s
            } else {
                "<non-string panic payload>"
            };

            writeln!(file, "{}", payload_str).unwrap_or_else(|_| {
                default_hook(info);
                std::process::exit(0);
            });
            writeln!(file, "{}", render_backtrace().sanitize_path()).unwrap_or_else(|_| {
                default_hook(info);
                std::process::exit(0);
            });

            let panic_msg = format!(
"Zenyx had a problem and crashed. To help us diagnose the problem you can send us a crash report.

We have generated a report file at \"{}\". Submit an issue or email with the subject of \"Zenyx Crash Report\" and include the report as an attachment.

To submit the crash report:

https://github.com/Zenyx-Engine/Zenyx/issues

We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.

Thank you kindly!", log_path.display());
            
            println!("{}", panic_msg.red().bold());
            println!("\nFor future reference, the error summary is as follows:\n{}", payload_str.red().bold());
            std::process::exit(0);
        }));
    });
}

fn render_backtrace() -> String {
    const HEX_WIDTH: usize = mem::size_of::<usize>() * 2 + 2;
    const NEXT_SYMBOL_PADDING: usize = HEX_WIDTH + 6;

    let mut backtrace = String::new();
    let bt = Backtrace::new();
    let symbols = bt
        .frames()
        .iter()
        .flat_map(|frame| {
            let symbols = frame.symbols();
            if symbols.is_empty() {
                vec![(frame, None, "<unresolved>".to_owned())]
            } else {
                symbols
                    .iter()
                    .map(|s| {
                        (
                            frame,
                            Some(s),
                            s.name()
                                .map(|n| n.to_string())
                                .unwrap_or_else(|| "<unknown>".to_owned()),
                        )
                    })
                    .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>();
    let begin_unwind = "rust_begin_unwind";
    let begin_unwind_start = symbols
        .iter()
        .position(|(_, _, n)| n == begin_unwind)
        .unwrap_or(0);
    for (entry_idx, (frame, symbol, name)) in symbols.iter().skip(begin_unwind_start).enumerate() {
        let ip = frame.ip();
        let _ = writeln!(backtrace, "{entry_idx:4}: {ip:HEX_WIDTH$?} - {name}");
        if let Some(symbol) = symbol {
            if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                let _ = writeln!(
                    backtrace,
                    "{:3$}at {}:{}",
                    "",
                    file.display(),
                    line,
                    NEXT_SYMBOL_PADDING
                );
            }
        }
    }
    backtrace
}

trait Sanitize {
    fn sanitize_path(&self) -> String;
}

impl Sanitize for str {
    fn sanitize_path(&self) -> String {
        let username_pattern = r"(?i)(/home/|/Users/|\\Users\\)([^/\\]+)";
        let re = Regex::new(username_pattern).expect("Failed to sanitize path, aborting operation");
        re.replace_all(self, "${1}<USER>").to_string()
    }
}
