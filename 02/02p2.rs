use std::io;


fn main() -> io::Result<()> {

    // so I think what we should do is to read each line, split it 
    // on spaces and then use a match on the command words.
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    loop{
        let mut input = String::new();
        let chars_read = io::stdin()
            .read_line(&mut input)?;

        if chars_read == 0 { // we are done
            println!("{}", horizontal * vertical);
            return Ok(());
        }

        let split :Vec<&str>  = input.split_whitespace().collect();
        assert_eq!(split.len(), 2);
        let command = split[0].trim();
        let magnitude = split[1].trim().parse::<i32>().expect("Could not parse magnitude as an int!");
        match command {
            "forward" => {
                horizontal = horizontal + magnitude;
                vertical = vertical + (aim * magnitude);
            },
            "up" => aim = aim - magnitude,
            "down" => aim = aim + magnitude,
            _ => println!("Command not understood! [{}]",command)
        }
    }

}
