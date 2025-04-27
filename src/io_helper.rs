use std::io::{stdout, Write};
use std::num::ParseIntError;
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::terminal::{Clear, ClearType};

pub fn print_result(result:i64){
    println!("Result: {result}");
}
pub fn print_result_float(result:f64){
    println!("Result: {result}");
}

pub fn flush_stdout() {
    stdout().flush().expect("Failed Flushing stdout");
}

pub fn clear_screen(){
    stdout().queue(Clear(ClearType::Purge)).unwrap();
    stdout().queue(MoveTo(0,0)).unwrap();
    flush_stdout();
}

pub fn pad_if_not_empty(operation_string: &str) -> String{
    let mut padded_operation_string = operation_string.to_string();
    if padded_operation_string.len() > 0 {
        padded_operation_string = padded_operation_string.to_owned() + "\n";
    }
    padded_operation_string
}

pub fn parse_to_i64(val:&String) -> Result<i64, ParseIntError>{
    val.trim_end().to_string().parse()
}