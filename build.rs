extern crate lalrpop;

fn main() {
    std::env::set_var("LALRPOP_LANE_TABLE", "disabled");
    lalrpop::Configuration::new()
        .always_use_colors()
        .log_verbose()
        .process_file("./src/parser/hc.lalrpop")
        .unwrap();
}
