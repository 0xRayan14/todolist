use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use termion::event::{Key};
use termion::input::TermRead;

fn main() -> io::Result<()> {
    let file = File::open("src/todolist.txt")?;
    let reader = BufReader::new(file);

    let mut first_five_lines: Vec<String> = Vec::with_capacity(5);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        first_five_lines.push(line.clone());

        if i == 4 {
            break;
        }
    }

    println!("\n 5 Last tasks :\n");
    for (i, line) in first_five_lines.iter().enumerate() {
        println!("{}.{}", i + 1, line);
    }

    loop {
        println!("\n [S] Show tasks \n [N] Add a task \n [T] Archive a task \n [A] Show archived tasks \n [D] Delete a task with its number \n [H] Help \n [Q] Quit \n");



        for c in io::stdin().keys() {
            match c.unwrap() {
                Key::Char('s') | Key::Char('S') => {
                    match File::open("src/todolist.txt") {
                        Ok(file) => {
                            println!("\n Your tasks :\n");
                            let reader = BufReader::new(file);
                            for (index, line) in reader.lines().enumerate() {
                                let line = line.unwrap();
                                println!("{}. {}", index + 1, line);
                            }
                        },
                        Err(error) => {
                            println!("Error opening file {}: {}", "src/todolist.txt \n", error);
                        },
                    }
                }


                Key::Char('n') | Key::Char('N') => {

                    let mut input = String::new();
                    println!("Add a task :");

                    io::stdin().read_line(&mut input)?;
                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("src/todolist.txt")?;

                    file.write_all(input.as_bytes())?;
                    println!("\n Task successfully added at (src/todolist.txt).");

                    match File::open("src/todolist.txt") {
                        Ok(file) => {
                            println!("\n Your tasks : \n");
                            let reader = BufReader::new(file);
                            for (index, line) in reader.lines().enumerate() {
                                let line = line.unwrap();
                                println!("{}. {}", index + 1, line);
                            }
                        },
                        Err(error) => {
                            println!("Error opening file {}: {}", "src/todolist.txt \n", error);
                        },
                    }
                }


                Key::Char('h') | Key::Char('H') =>  println!("\n [S] Show tasks \n [N] Add a task \n [T] Archive a task \n [A] Show archived tasks \n [D] Delete a task with its number \n [H] Help \n [Q] Quit \n"),


                Key::Char('a') | Key::Char('A') => {
                    match File::open("src/archived.txt") {
                        Ok(file) => {
                            println!("Archived tasks : \n");
                            let reader = BufReader::new(file);
                            for (index, line) in reader.lines().enumerate() {
                                let line = line.unwrap();
                                println!("{}. {}", index + 1, line);
                            }
                        },
                        Err(error) => {
                            println!("Error opening file {}: {}", "src/archived.txt \n", error);
                        },
                    }
                }


                Key::Char('d') | Key::Char('D') => {
                    // Ask the user for the line number to delete.
                    let mut line_number_to_delete = String::new();
                    println!("Enter the line number to delete:");
                    io::stdin().read_line(&mut line_number_to_delete)?;

                    // Parse the user input as an integer.
                    let line_to_delete = line_number_to_delete.trim().parse::<usize>();

                    match line_to_delete {
                        Ok(line_to_delete) => {
                            // Open the file for reading and writing.
                            let input = File::open(&"src/todolist.txt")?;
                            let reader = BufReader::new(&input);

                            // Create a temporary Vec to store lines.
                            let mut lines: Vec<String> = Vec::new();

                            // Iterate through the lines and skip the line to delete.
                            let mut line_number = 1;
                            for line in reader.lines() {
                                if line_number != line_to_delete {
                                    if let Ok(line_text) = line {
                                        lines.push(line_text);
                                    }
                                }
                                line_number += 1;
                            }

                            // Reopen the file for writing.
                            let mut file = File::create(&"src/todolist.txt")?;

                            // Write the modified lines back to the original file.
                            for line in lines {
                                writeln!(&mut file, "{}", line)?;
                            }

                            println!("Line {} deleted from the file. \n", line_to_delete);
                        }
                        Err(_) => {
                            println!("Invalid input. Retry and enter a valid line number. \n");
                        }
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