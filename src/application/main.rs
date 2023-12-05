mod converter;
mod utils;

use std::path::Path;
use converter::convert_jpg_to_pdf;
use utils::parse_arguments;
// TODO: FIX ERRORS
fn main() {
    let args = parse_arguments();

    convert_jpg_to_pdf(Path::new(&args.input_path), Path::new(&args.output_path));
}

