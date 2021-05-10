use std::{str::FromStr, string::ParseError};

/// sscanf 宏实现，参考：https://gist.github.com/murasesyuka/50f79a0f0dc0da72a4f3ce4429001060
#[macro_use]
macro_rules! sscanf {
    ($str: expr, $( $t:ty ),+ ) => {{
        let mut iter = $str.split_whitespace();
        ($(iter.next().unwrap().parse::<$t>().unwrap(),)+)
    }};
}

pub enum StatementEnum {
    Insert,
    Select,
    Unknown,
}

pub struct Statement {
    pub typ: StatementEnum,
    pub row: Option<UserRow>,
}

impl Statement {
    pub fn new() -> Statement {
        Statement {
            typ: StatementEnum::Unknown,
            row: None,
        }
    }
}

#[derive(Debug)]
pub struct UserRow {
    id: usize,
    username: String,
    email: String,
}

impl UserRow {
    fn new(id: usize, username: String, email: String) -> UserRow {
        UserRow {
            id: id,
            username: username,
            email: email,
        }
    }
}

/// 解析语句
pub fn prepare_statement(input: &str, statem: &mut Statement) {
    if input.starts_with("insert") {
        statem.typ = StatementEnum::Insert;
        // insert 1 cstack foo@bar.com
        let v_tuple = sscanf!(input, String, usize, String, String);
        statem.row = Some(UserRow::new(v_tuple.1, v_tuple.2, v_tuple.3));
    } else if input.starts_with("select") {
        statem.typ = StatementEnum::Select;
    }

    return;
}

/// 执行语句
pub fn execute_statement(stmt: &Statement) {
    match stmt.typ {
        StatementEnum::Insert => {
            println!("This is where we would do an insert");
        }
        StatementEnum::Select => {
            println!("This is where we would do an select");
        }
        StatementEnum::Unknown => {
            println!("Unknown statement.");
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_sscanf1() {
        // https://cstack.github.io/db_tutorial/parts/part3.html
        let s1 = "insert 1 cstack foo@bar.com";
        let v1 = sscanf!(s1, String, u32, String, String);
        println!("{:?}", v1);
    }
}
