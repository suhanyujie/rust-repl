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

/// 解析语句
pub fn prepare_statement(input: &str, statem: &mut Statement) {
    if input.starts_with("insert") {
        statem.typ = StatementEnum::Insert;
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

// todo
// pub fn sscanf(input: &str, fmt: &str, params...: D) {}
