use std::fs::File;
use std::io::{self, BufRead, BufReader};
use termion::event::{self, Key};
use termion::input::TermRead;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let file = File::open("src/todolist")?;
    let reader = BufReader::new(file);

    let mut first_five_lines: Vec<String> = Vec::with_capacity(5);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        first_five_lines.push(line.clone());

        if i == 4 {
            break;
        }
    }

    for (i, line) in first_five_lines.iter().enumerate() {
        println!("{}.{}", i + 1, line);
    }



    loop {
        println!("A: Archived ToDo's, N: Add a ToDo, H: Help, Q: Quit");

        for c in io::stdin().keys() {
            match c.unwrap() {
                Key::Char('h') | Key::Char('H') => println!("A: Archived ToDo's, N: Add a ToDo, Q: Quit"),

                Key::Char('a') | Key::Char('A') => {
                    for (i, line) in first_five_lines.iter().enumerate() {
                        println!("{}.{}", i + 1, line);
                    }
                }
                Key::Char('q') | Key::Char('Q') => {
                    break;
                }
                _ => {}
            }
        }

        break;
    }

    Ok(())
}
