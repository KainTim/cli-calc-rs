use std::io::{stdout, Write};
use std::num::{ParseFloatError};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::terminal::{Clear, ClearType};

pub fn print_result(result:f64){
    println!("Result: {result:.3}");
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

pub fn parse_to_f64(val:&String) -> Result<f64, ParseFloatError>{
    val.trim_end().to_string().parse()
}