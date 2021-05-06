extern crate clap;

use clap::{App, Arg, SubCommand};
use std::io::Error;
use std::process::{Command, Stdio};

static cli_name: &str = "simpleREPL";

fn print_prompt() {
    println!("{} >", cli_name);
}

fn display_help() {
    println!("Welcom to {}! These are avaliable commands:", cli_name);
    println!(".help    - Show available commands");
    println!(".clear   - Clear the terminal screen");
    println!(".exit    - Closes your connection to {}", cli_name);
}

fn clear_screen() {
    let os_stdout = std::io::stdout();
    Command::new("clear")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("exec clear error.");
}

pub fn repl_run() -> Result<(), Error> {
    Ok(())
}

pub fn get_app<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("rs-repl")
        .version("0.1.0")
        .author("suhanyujie<suhanyujie@qq.com>")
        .about("REPL in Rust")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("note")
                .about("some operation for note")
                .arg(
                    Arg::with_name("create")
                        .takes_value(false)
                        .help("create one note post"),
                ),
        );
    // .get_matches();
    return app;
}

pub fn handle_app(app: App) -> Result<(), clap::Error> {
    let arg_matchs = app.get_matches();
    if let Some(sub_m) = arg_matchs.subcommand_matches("note") {
        if let Some(key_value_str) = sub_m.value_of("create") {
            let value = get_param(key_value_str);
            match value {
                Some(val) => {
                    eprintln!("this is create: {}", val);
                }
                val => {
                    eprintln!("no run {:?}", val);
                }
            }
        } else {
            return Err(clap::Error {
                message: "invalid param...".to_string(),
                kind: clap::ErrorKind::InvalidValue,
                info: Some(vec![]),
            });
        }
    } else {
    }
    Ok(())
}

/// 用 `=` 号将字符串分割成一个键值对，并返回其中的值。
pub fn get_param<'a>(param_str: &'a str) -> Option<&'a str> {
    let arr: Vec<&'a str> = param_str.splitn(2, '=').collect();
    if arr.len() == 2 {
        return Some(arr[1]);
    } else {
        None
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_clear_cmd() {
        clear_screen();
    }
}
