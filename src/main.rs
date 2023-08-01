use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(short = 'm', long = "message", default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[clap(short = 'd', long = "dead")]
    ///Make the cat appear dead
    dead: bool,

    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    cat_file: Option<std::path::PathBuf>,
}

fn main() {
    let options = Options::parse();
    let message = options.message;
    let eye = if options.dead { "x" } else { "o" };

    match &options.cat_file {
        Some(path) => {
            let cat_template =
                std::fs::read_to_string(path).expect(&format!("could not read file {:?}", path));
            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", &cat_picture);
        }
        None => {
            if message.to_lowercase() == "woof" {
                eprintln!("A cat should not bark like a dog.")
            }
            println!("Cat Message -> {cat_message}", cat_message = message);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!(" \\");
            println!(" /\\_/\\");
            println!(" ( {eye} {eye} )", eye = eye.red().bold());
            println!(" =( I )=");
        }
    }
}
