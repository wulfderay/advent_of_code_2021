use std::cmp::Ordering;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut last_num = 0;
    let mut total_increased = -1;
    loop {
        let bytes_read = io::stdin()
            .read_line(&mut buffer)?;

        if bytes_read == 0 {
            println!("{}", total_increased);
            return Ok(())
        }

        let the_int = buffer.trim().parse::<i32>().expect("Failed to parse as int!");
        match last_num.cmp(&the_int){
            Ordering::Less => {
                total_increased =total_increased + 1;


            },
            _ => {}
        }
        buffer = String::new();
        last_num = the_int;
    }
}



