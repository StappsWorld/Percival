extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .always_use_colors()
        .process_file("./src/parser/hc.lalrpop")
        .unwrap();
}
