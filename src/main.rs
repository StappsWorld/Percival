pub mod parser;
use std::{
    fs::{self, File},
    io::{self, stdin, stdout, Read, Write},
    path::PathBuf,
};

fn get_index(available_examples: &Vec<PathBuf>) -> io::Result<usize> {
    let mut index = usize::MAX;
    let mut input = String::new();

    while index >= available_examples.len() {
        print!(
            "Which would you like to run (a number 0 - {}): ",
            available_examples.len() - 1
        );
        stdout().flush()?;

        stdin().read_line(&mut input)?;

        match usize::from_str_radix(&input.trim(), 10) {
            Ok(idx) if idx < available_examples.len() => index = idx,
            Ok(idx) => {
                println!("{} is out of range, try again.", idx);
            }
            _ => {
                println!("{:?} is not a valid number, try again.", input.trim());
            }
        }

        input.clear();
    }

    Ok(index)
}

fn main() -> io::Result<()> {
    let available_examples = fs::read_dir("./examples")?
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<PathBuf>>();

    println!("Available Examples: ");

    available_examples
        .iter()
        .enumerate()
        .for_each(|(idx, path)| println!("\t{}: {:?}", idx, path.file_name().unwrap()));

    let user_index = get_index(&available_examples)?;

    let selected_example_path = available_examples.get(user_index).unwrap();
    println!(
        "You selected:\n\t{}: {:?}",
        user_index, selected_example_path
    );

    let mut file_contents = String::new();

    let nbytes = File::open(selected_example_path)?.read_to_string(&mut file_contents)?;

    println!("Read {} bytes from {:?}", nbytes, selected_example_path);
    println!("Parsing...");
    println!("Result: {:#?}", parser::Parser::new().parse(&mut vec![],&file_contents,));
    Ok(())
}
