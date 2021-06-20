pub mod lexer;
pub mod parser;

fn main() -> std::io::Result<()> {
    let file_contents = r#"
class Beans;
Beans *beans = 5 * 5.0;
F64 float = 5.0;
U8 *string = "Hello world!";
U8 *chars = 'Hello world!';
    "#;
    let mut types = vec![];
    println!("{}", file_contents);
    println!(
        "{:#?}",
        parser::Parser::new().parse(&mut types, lexer::Lexer::new(file_contents))
    );

    println!("Defined types: {:#?}", types);
    Ok(())
}
