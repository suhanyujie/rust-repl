extern crate clap;

use std::process::Command;
use std::{collections::HashMap, io::Error};

use crate::statement::{self, Statement};

static CLI_NAME: &str = "simpleREPL";

/// 打印提示信息
fn print_prompt() {
    println!("{} >", CLI_NAME);
}

// 元指令匹配结果

#[derive(PartialEq, Eq)]
pub enum MetaCmdResEnum {
    MetaCmdSuccess,
    MegaCmdUnrecognized,
}

/// 运行 REPL 主体逻辑
pub fn repl_run() -> Result<(), Error> {
    // 从标准输入读取输入
    let mut input = String::new();
    loop {
        input.clear();
        print_prompt();
        let res = std::io::stdin().read_line(&mut input);
        if res.is_err() {
            println!("{:?}", res);
            break;
        }
        let input_str = clean_input(&*input);
        println!("user input is: {}", input_str);
        if input_str.starts_with('.') {
            // build-in command
            if check_is_quit(input_str) {
                println!("cli app terminate...");
                break;
            }
            let res = do_meta_command(input_str);
            match res {
                MetaCmdResEnum::MegaCmdUnrecognized => {
                    println!("Unrecognized command: {}", input_str);
                    continue;
                }
                MetaCmdResEnum::MetaCmdSuccess => {
                    continue;
                }
            }
        } else {
            // statement
            let mut stmt = Statement::new();
            statement::prepare_statement(input_str, &mut stmt);
            statement::execute_statement(&stmt);
        }
    }
    Ok(())
}

// 处理内置指令
fn do_meta_command(cmd_text: &str) -> MetaCmdResEnum {
    if check_is_quit(cmd_text) {
        std::process::exit(0);
        return MetaCmdResEnum::MetaCmdSuccess;
    }
    let mut commands: HashMap<&'static str, fn()> = HashMap::new();
    set_meta_commands(&mut commands);
    let handle = commands.get(cmd_text);
    if handle.is_none() {
        return MetaCmdResEnum::MegaCmdUnrecognized;
    }
    handle.unwrap()();
    MetaCmdResEnum::MetaCmdSuccess
}

fn clean_input(input: &str) -> &str {
    input.trim_end()
}

// 设置内置命令的处理
pub fn set_meta_commands(cmd_map: &mut HashMap<&'static str, fn()>) {
    cmd_map.insert(".help", cmd_help);
    cmd_map.insert(".clear", cmd_clear);
    cmd_map.insert(".quit", cmd_quit);
}

// 检查用户输入的指令是否是推出指令
fn check_is_quit(cmd_text: &str) -> bool {
    cmd_text == ".quit"
}

// clear 命令
fn cmd_clear() {
    if Command::new("clear").status().unwrap().success() {
        println!("screen successfully cleared");
    }
}

/// 显示帮助信息
fn cmd_help() {
    println!("Welcom to {}! These are avaliable commands:", CLI_NAME);
    println!(".help    - Show available commands");
    println!(".clear   - Clear the terminal screen");
    println!(".quit    - Closes your connection to {}", CLI_NAME);
}

fn cmd_quit() {}

mod tests {
    use super::*;

    #[test]
    fn test_clear_cmd() {
        cmd_clear();
    }
}
