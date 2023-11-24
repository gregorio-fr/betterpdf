use std::env;

pub struct Arguments {
    pub input_path: String,
    pub output_path: String,
}

pub fn parse_arguments() -> Arguments {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: betterpdf <input.jpg> <output.pdf>");
    }
    Arguments {
        input_path: args[1].clone(),
        output_path: args[2].clone(),
    }
}
