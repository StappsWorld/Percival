pub mod parser;
use std::{
    fs,
    io::{self, Read, Write},
    path::PathBuf,
};

fn get_examples() -> io::Result<Vec<PathBuf>> {
    let available_examples = fs::read_dir("./examples")?
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<PathBuf>>();

    Ok(available_examples)
}

fn get_index(available_examples: &Vec<PathBuf>) -> io::Result<usize> {
    let mut index = usize::MAX;

    while index >= available_examples.len() {
        print!(
            "Which would you like to run (a number 0 - {}): ",
            available_examples.len() - 1
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        index = usize::from_str_radix(&input.trim(), 10).unwrap();

        if index >= available_examples.len() {
            println!("{} is out of range, try again.", index);
        }
    }

    Ok(index)
}

fn main() -> io::Result<()> {
    let available_examples = get_examples()?;

    println!("Available Examples: ");

    available_examples
        .iter()
        .enumerate()
        .for_each(|(idx, path)| println!("\t{}: {:?}", idx, path.file_name().unwrap()));

    let index = get_index(&available_examples)?;

    let example_path = available_examples.get(index).unwrap();
    println!("You selected:\n\t{}: {:?}", index, example_path);

    let mut file_contents = String::new();

    let nbytes = fs::File::open(example_path)?.read_to_string(&mut file_contents)?;
    println!("Read {} bytes from {:?}", nbytes, example_path);

    println!("Parsing...");
    println!("Result: {:#?}", parser::Parser::new().parse(&file_contents));
    Ok(())
}
