mod command;
mod statement;

use command::repl_run;
use std::fmt::format;

fn main() {
    let res: Result<(), std::io::Error> = repl_run();
    if res.is_err() {
        println!("repl error: {}", format(format_args!("{:?}", res)));
    }
}
