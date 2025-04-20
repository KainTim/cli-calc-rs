use std::io::{stdin, stdout, Write};
use std::num::ParseIntError;
use crossterm::cursor::{MoveTo, MoveUp};
use crossterm::{execute, QueueableCommand};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

fn main() {
    //Remember History before Application Start
    execute!(stdout(), EnterAlternateScreen).expect("Error while Entering Alternate Terminal Screen");
    
    clear_screen();
    
    let mut input: String = String::new();
    
    loop{

        println!("\
        1 -> Add\n\
        2 -> Subtract\n\
        3 -> Multiply\n\
        4 -> Divide\n\
        0 -> Exit");

        flush_stdout();
        input.clear();

        let val: i32 = read_nr_from_stdin("Selection", false, String::new());

        clear_screen();

        match val {
            1 => {
                println!("Result {}", add(get_2_values("Addition")));
            }
            2 => {
                println!("Result {}", subtract(get_2_values("Subtraction")));
            }
            3 => {
                println!("Result {:?}", multiply(get_2_values("Multiplication")));
            }
            4 => {
                println!("Result {:?}", divide(get_2_values("Division")));
            }
            0 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("No such menu option!");
                println!("Press Return to retry")
            }
        }

        stdin().read_line(&mut input).expect("Failed reading from stdin");

        clear_screen();
    }
    //Restore History
    execute!(stdout(), LeaveAlternateScreen).expect("Error while Leaving Alternate Terminal Screen");
}

fn flush_stdout() {
    stdout().flush().expect("Failed Flushing stdout");
}

fn clear_screen(){
    stdout().queue(Clear(ClearType::Purge)).unwrap();
    stdout().queue(MoveTo(0,0)).unwrap();
    flush_stdout();
}
fn get_2_values(operation_string:&str) ->(i32,i32,){
    let padded_operation_string = pad_if_not_empty(operation_string);
    let val1:i32 = read_nr_from_stdin("First Number",true, padded_operation_string.clone());
    let val2:i32 = read_nr_from_stdin("Second Number",true, padded_operation_string);

    clear_screen();

    (val1,val2)
}
fn read_nr_from_stdin(out_string:&str, screen_should_clear:bool, operation_string: String) ->i32{
    loop {
        if screen_should_clear {
            clear_screen();
        }
        print!("{}", operation_string);
        let mut val_string: String = String::new();
        print!("{}: ", out_string);
        flush_stdout();
        stdin().read_line(&mut val_string).expect("Failed reading from stdin");
        return match parse_to_i32(&val_string) {
            Ok(nr) => nr,
            Err(_err) => {
                if screen_should_clear {
                    clear_screen();
                    print!("{}", operation_string);
                }
                println!("Number was not convertible to an i32!");
                println!("Press Return to retry");
                stdin().read_line(&mut val_string).expect("Failed reading from stdin");
                stdout().queue(MoveUp(1)).unwrap();
                continue;
            },
        };
    }
}

fn pad_if_not_empty(operation_string: &str) -> String{
    let mut padded_operation_string = operation_string.to_string();
    if padded_operation_string.len() > 0 {
        padded_operation_string = padded_operation_string.to_owned() + "\n";
    }
    padded_operation_string
}

fn parse_to_i32(val:&String) -> Result<i32, ParseIntError>{
    val.trim_end().to_string().parse()
}

fn add(values:(i32,i32)) ->i32{
    values.0+values.1
}
fn subtract(values:(i32,i32)) ->i32{
    values.0-values.1
}
fn multiply(values:(i32,i32)) ->i64{
    let values_bigger:(i64,i64) = (values.0 as i64, values.1 as i64);
    values_bigger.0 * values_bigger.1
}
fn divide(values:(i32,i32)) ->f32{
    if values.1 == 0 {
        println!("Divide by 0 not allowed!");
        return -1f32;
    }
    (values.0 / values.1) as f32
}