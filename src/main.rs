use std::fmt::format;

use command::{get_app, handle_app, repl_run};

mod command;

fn main() {
    let res: Result<(), std::io::Error> = repl_run();
    if res.is_err() {
        println!("repl error: {}", format(format_args!("{:?}", res)));
    }
}
