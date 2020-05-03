use std::io::{self, Write};
use std::process::exit;
use std::collections::HashMap;
use crate::PrepareResult::{PrepareUnrecognizedCommand, PrepareSuccess};
use crate::MetaCommandResult::MetaCommandUnrecognizedCommand;
use crate::StatementType::{StatementInsert, StatementSelect, StatementUnknown};
use crate::ExecuteResult::{ExecuteFailed, ExecuteSuccess};

const NUM_ROWS_PER_PAGE:usize = 100;

struct Table {
    rows: HashMap<String, Row>
}

impl Table {
    fn new() -> Table {
        Table {
            rows: HashMap::new()
        }
    }
}

type Row = (String, String);

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand
}

enum ExecuteResult {
    ExecuteSuccess,
    ExecuteFailed
}

enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedCommand
}

enum StatementType {
    StatementUnknown,
    StatementSelect,
    StatementInsert
}

struct Statement {
    statement_type: StatementType
}

fn print_prompt(){
    print!("db> ");
    io::stdout().flush().unwrap();
}

fn get_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    user_input
}

fn do_meta_command(meta_command: &str) -> MetaCommandResult {
    match meta_command {
        ".exit" => exit(0),
        _ => MetaCommandUnrecognizedCommand
    }
}

fn prepare_statement(user_command: &str, statement: &mut Statement) -> PrepareResult {
    // Take the statement up to the first space, uppercase and trim it
    // so it can be used to determine the statement type
    let statement_parts: Vec<&str> = user_command.split(' ').collect();
    let statement_type = String::from(statement_parts[0]);
    let statement_type = statement_type.to_uppercase();
    let statement_type = statement_type.trim();

    match statement_type {
        "SELECT" => {
            statement.statement_type = StatementSelect;
            PrepareSuccess
        },
        "INSERT" => {
            statement.statement_type = StatementInsert;
            PrepareSuccess
        },
        _ => {
            statement.statement_type = StatementUnknown;
            PrepareUnrecognizedCommand
        }
    }
}

fn execute_insert(statement: &str, table: &mut Table) -> ExecuteResult {
    let arguments:Vec<&str> = statement.split(' ').collect();
    if arguments.len() < 4 {
        println!("Not enough arguments");
        return ExecuteFailed
    }
    table.rows.insert(String::from(arguments[1]), (String::from(arguments[2]), String::from(arguments[3])));
    ExecuteSuccess
}

fn execute_select(statement: &str, table: &mut Table) -> ExecuteResult {
    for (id, data) in &table.rows {
        println!("{}  | {} | {}", id, data.0, data.1);
    }
    ExecuteSuccess
}

fn execute_statement(statement: &Statement, user_input: &str, table: &mut Table) -> ExecuteResult {
    match statement.statement_type {
        StatementInsert => execute_insert(user_input, table),
        StatementSelect => execute_select(user_input, table),
        StatementUnknown => panic!("Something went wrong! The Statement Type is Unknown")
    }
}

fn main(){
    let mut table = Table::new();
    loop{
        print_prompt();
        let user_input = get_input();
        let user_input = user_input.trim();

        // Check if it's a meta command and handle the meta command
        if user_input.starts_with('.'){
            do_meta_command(user_input);
            continue
        }
        let mut statement = Statement {statement_type: StatementUnknown};
        let prepare_result = prepare_statement(user_input, &mut statement);
        match prepare_result {
            PrepareSuccess => {
                match execute_statement(&statement, user_input, &mut table) {
                    ExecuteSuccess => println!("Executed"),
                    ExecuteFailed => println!("Failed to Execute")
                }
                continue
            },
            PrepareUnrecognizedCommand => {
                println!("Syntax Error Near '{}'", user_input);
                continue
            }
        }
    }
}

