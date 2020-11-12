use std::{env, process};
use structopt::StructOpt;

use todoist_add as todoist;

#[derive(Debug, StructOpt)]
#[structopt(name = "todoist", about = "The CLI for the Todoist quick add")]
struct Opt {
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Add {
        #[structopt(short = "q")]
        content: String,
    },
}

fn main() {
    let token;
    match env::var("token") {
        Ok(val) => token = val,
        Err(_e) => {
            println!("Token not found. Please define it as the $token environment variable");
            process::exit(0);
        }
    }

    let opt = Opt::from_args();
    match opt.cmd {
        Command::Add { content } => {
            if let Ok(result) = todoist::add(token, content) {
                println!("{}", result);
            }
        }
    };
}
