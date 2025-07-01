use clap::Parser;
use std::error::Error;

#[derive(Parser)]
#[command(
    name = "sql-param",
    about = "A tool for automatically replacing sql placeholders",
    version,
    long_about = "SQL-Param is a TUI tool that helps you replace SQL placeholders with actual values. 
    It can automatically parse clipboard content for SQL statements and parameters from server logs, 
    then generate the final SQL with replaced placeholders."
)]
pub struct Argument {
    /// SQL statement with placeholders. Example: 'UPDATE user SET name = ?, age = ?, update_time = ?, id_card = ? WHERE id = ? AND deleted = ?;'
    #[arg(short, long)]
    sql: Option<String>,

    /// Parameter values (comma-separated). Example: 'zhangsan(String), 18(Integer), 2025-06-13 16:44:56.499(Timestamp), 123456789(Long), 1(Integer), 0(Integer)'
    #[arg(short, long)]
    value: Option<String>,
}

pub fn parse_arguments() -> Option<Result<(), Box<dyn Error>>> {
    let cli = Argument::parse();

    if let (Some(sql), Some(value)) = (&cli.sql, &cli.value) {
        let result = crate::core::replace_placeholder(sql, value);
        println!("{}", result);
        return Some(Ok(()));
    }

    if cli.sql.is_some() || cli.value.is_some() {
        eprintln!("Error: Both --sql and --value must be provided together");
        eprintln!("Use --help for more information");
        std::process::exit(1);
    }

    None
}
