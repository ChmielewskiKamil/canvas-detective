use canvas_detective::cli::{create_canvas_file, Args};

fn main() {
    Args::new()
        .map(|args| (args.input_path, args.output_path))
        .and_then(|(input_path, output_path)| create_canvas_file(&input_path, &output_path))
        .map(|_| println!("Canvas file created successfully"))
        .unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            std::process::exit(1);
        })
}
