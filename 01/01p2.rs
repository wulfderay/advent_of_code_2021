use std::cmp::Ordering;
use std::io;

fn main() -> io::Result<()> {

    // mu core assumption is wrong.. In order to have a sliding window
    // we need to keep track of the individual inputs so that we can 
    // window properly (removing the earliest item and adding the latest).
    const WINDOW_SIZE : usize = 3;
    let mut total_increased = 0;
    let mut window_start = 0;
    let mut window_buf = [0;WINDOW_SIZE];
    let mut records = 0;
    loop {
        
        let mut buffer = String::new();
        let bytes_read = io::stdin()
            .read_line(&mut buffer)?;

        if bytes_read == 0 {
            println!("{}", total_increased);
            return Ok(())
        }

        records = records +1;

        let mut last_window = 0;
        let mut current_window = 0;

        let the_int = buffer.trim().parse::<i32>().expect("Failed to parse as int!");
        for n in 0..WINDOW_SIZE {
            last_window = last_window + window_buf[(window_start + n) % WINDOW_SIZE];
        }
        window_start = (window_start + 1) % WINDOW_SIZE;
        window_buf[(window_start + WINDOW_SIZE -1) % WINDOW_SIZE] = the_int;
        for n in 0..WINDOW_SIZE {
            current_window = current_window + window_buf[(window_start + n) % WINDOW_SIZE];
        }

        //println!("last window: {} This window {}", last_window, current_window);
        if records <= WINDOW_SIZE {
            continue;
        }
        match last_window.cmp(&current_window){
            Ordering::Less => {
                total_increased =total_increased + 1;
            },
            _ => {}
        }
    } 
}



