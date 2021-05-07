use std::fmt::format;

use command::repl_run;

mod command;

fn main() {
    let res: Result<(), std::io::Error> = repl_run();
    if res.is_err() {
        println!("repl error: {}", format(format_args!("{:?}", res)));
    }
}
