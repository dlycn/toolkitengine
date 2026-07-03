use clap::builder::StyledStr;
use clap::{Parser, CommandFactory};
use std::io::{self, Write};
use std::path::PathBuf;

const  HELP: &str = "It looks like you tried to open the CLI program, Here is its help information below.\n
Of course,the program name you customize should have a higher priority than gocmd.";

//const IMGMODE: [&str;2] = ["ETC1S","UASTC"];
#[derive(Debug, Parser)]
#[command(name = "gocmd", version, about = String::from(HELP), long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("user"))]
    name: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
    #[arg(short, long)]
    dir: Option<PathBuf>,
    #[arg(short = 'v', long, action = clap::ArgAction::Version, hide = true)]
    version_v: (),
}

fn itf_default(sstr:StyledStr){
        println!("{}", sstr);
        println!("Now you can press the Enter key to exit this interface.Try again and enjoy it!");
        io::stdout().flush().unwrap();   // 确保提示立即显示
        let mut dummy = String::new();
        io::stdin().read_line(&mut dummy).unwrap();
}

fn itf_main(args: Args){
    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}

fn main() {
    let mut cmd: clap::Command = Args::command();
    let args: Args = Args::parse();

    // 检查是否没有任何命令行参数（程序名除外）
    if std::env::args().len() == 1 {
        itf_default(cmd.render_help())
    }
    else {
        itf_main(args)
    }
}