mod math_helper;
mod io_helper;

use crate::io_helper::{clear_screen, flush_stdout, pad_if_not_empty, parse_to_i64, print_result, print_result_float};
use crate::math_helper::{add, divide, multiply, subtract};
use crossterm::cursor::MoveUp;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, QueueableCommand};
use std::io::{stdin, stdout};

fn main() {
    //Remember History before Application Start
    execute!(stdout(), EnterAlternateScreen).expect("Error while Entering Alternate Terminal Screen");
    
    clear_screen();
    let mut input:String = String::new();

    loop{

        println!("\
        1 -> Add\n\
        2 -> Subtract\n\
        3 -> Multiply\n\
        4 -> Divide\n\
        0 -> Exit");

        flush_stdout();

        let val: i64 = read_nr_from_stdin("Selection", false, "");

        clear_screen();

        match val {
            1 => {
                print_result(add(get_2_values("Addition")));
            }
            2 => {
                print_result(subtract(get_2_values("Subtraction")));
            }
            3 => {
                print_result(multiply(get_2_values("Multiplication")));
            }
            4 => {
                print_result_float(divide(get_2_values("Division")));
            }
            0 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("No such menu option!\n\
                          Press Return to retry");
            }
        }
        stdin().read_line(&mut input).expect("Failed reading from stdin");

        clear_screen();
    }
    //Restore History
    execute!(stdout(), LeaveAlternateScreen).expect("Error while Leaving Alternate Terminal Screen");
}
fn get_2_values(operation_string:&str) ->(i64,i64,){
    let padded_operation_string = pad_if_not_empty(operation_string);
    let val1:i64 = read_nr_from_stdin("First Number",true, padded_operation_string.as_str());
    let val2:i64 = read_nr_from_stdin("Second Number",true, padded_operation_string.as_str());

    clear_screen();

    (val1,val2)
}
fn read_nr_from_stdin(prompt_string:&str, screen_should_clear:bool, operation_string: &str) ->i64{
    let mut val_string: String = String::new();
    loop {
        if screen_should_clear {
            clear_screen();
        }
        print!("{operation_string}{prompt_string}: ");
        flush_stdout();
        stdin().read_line(&mut val_string).expect("Failed reading from stdin");
        return match parse_to_i64(&val_string) {
            //Returns the value if it is OK
            Ok(nr) => nr,
            //Runs the below code and does not return a value and instead continues executing the loop
            Err(_err) => {
                if screen_should_clear {
                    clear_screen();
                    //only operation as prompt gets printed in next loop
                    print!("{}", operation_string);
                }
                println!("Number was not convertible to an i64!\n\
                          Press Return to retry");
                stdin().read_line(&mut val_string).expect("Failed reading from stdin");
                // Move the cursor up so that there is no New Line in between selection attempts
                stdout().queue(MoveUp(1)).unwrap();
                val_string.clear();
                //doesn't return a value
                continue;
            },
        };
    }
}