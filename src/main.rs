use indicatif::ProgressBar;
use std::fs;
use std::path::Path;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Input directory
   #[arg(short, long)]
   input: String,

   /// Output directory
   #[arg(short, long)]
   output: String,

   /// Output Size
   #[arg(short, long)]
   size: u32,
}

fn main() {
    // Get the arguments
    let args = Args::parse();

    // Get a list of all PNG files in the input directory
    let files: Vec<_> = fs::read_dir(args.input)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension() == Some("png".as_ref()) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Create a progress bar with a custom style
    let pb = ProgressBar::new(files.len() as u64);

    // Loop through the list of files
    for file in files {
        // Prevent borrowing after move
        let file_copy = file.clone();

        // Read input image
        let img = image::open(file).unwrap();

        // Downsample image
        let downsampled_img = img.resize(args.size, args.size, image::imageops::FilterType::Triangle);

        // Get the file name without the extension
        let filename = file_copy
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        // Save image to output file in the desired directory
        let out_path = Path::new(&args.output).join(format!("{}.jpg", filename));
        downsampled_img.save(out_path).unwrap();

        // Increment the progress bar
        pb.inc(1);
    }

    // Finish the progress bar
    pb.finish();
}

