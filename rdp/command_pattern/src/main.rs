// Command pattern is used to separate out actions into its own objects and pass them
// as parameters.

// Approach: Using trait objects
pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}

pub struct CreateTable;
impl Migration for CreateTable {
    fn execute(&self) -> &str {
        "create table"
    }
    fn rollback(&self) -> &str {
        "drop table"
    }
}

pub struct AddField;
impl Migration for AddField {
    fn execute(&self) -> &str {
        "add field"
    }
    fn rollback(&self) -> &str {
        "remove field"
    }
}

struct Schema {
    commands: Vec<Box<dyn Migration>>,
}

impl Schema {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }

    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }

    fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| cmd.rollback())
            .collect()
    }
}

// Approach: Using function pointers
type FnPtr = fn() -> String;
struct Command_2 {
    execute: FnPtr,
    rollback: FnPtr,
}

struct Schema_2 {
    commands: Vec<Command_2>,
}

impl Schema_2 {
    fn new() -> Self {
        Self { commands: vec![] }
    }
    fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
        self.commands.push(Command_2 { execute, rollback });
    }
    fn execute(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }
    fn rollback(&self) -> Vec<String> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

fn add_field() -> String {
    "add field".to_string()
}

fn remove_field() -> String {
    "remove field".to_string()
}

// Approach: Using Fn trait objects
type Migration_3<'a> = Box<dyn Fn() -> &'a str>;
struct Schema_3<'a> {
    executes: Vec<Migration_3<'a>>,
    rollbacks: Vec<Migration_3<'a>>,
}

impl<'a> Schema_3<'a> {
    fn new() -> Self {
        Self {
            executes: vec![],
            rollbacks: vec![],
        }
    }
    fn add_migration<E, R>(&mut self, execute: E, rollback: R)
    where
        E: Fn() -> &'a str + 'static,
        R: Fn() -> &'a str + 'static,
    {
        self.executes.push(Box::new(execute));
        self.rollbacks.push(Box::new(rollback));
    }
    fn execute(&self) -> Vec<&str> {
        self.executes.iter().map(|cmd| cmd()).collect()
    }
    fn rollback(&self) -> Vec<&str> {
        self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
    }
}

fn add_field_3() -> &'static str {
    "add field"
}

fn remove_field_3() -> &'static str {
    "remove field"
}

fn main() {
    // Approach: trait objects
    let mut schema = Schema::new();

    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());

    // Approach: function pointers
    let mut schema_2 = Schema_2::new();
    schema_2.add_migration(|| "create table".to_string(), || "drop table".to_string());
    schema_2.add_migration(add_field, remove_field);
    assert_eq!(vec!["create table", "add field"], schema_2.execute());
    assert_eq!(vec!["remove field", "drop table"], schema_2.rollback());

    // Approach: using fn trait objects
    let mut schema_3 = Schema_3::new();
    schema_3.add_migration(|| "create table", || "drop table");
    schema_3.add_migration(add_field_3, remove_field_3);
    assert_eq!(vec!["create table", "add field"], schema_3.execute());
    assert_eq!(vec!["remove field", "drop table"], schema_3.rollback());
}
