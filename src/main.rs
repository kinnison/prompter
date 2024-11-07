use clap::Parser;
use cli::Command;

mod cli;
mod config;
mod sources;

fn main() {
    let config = config::Config::default();
    let cli = cli::Cli::parse();
    if matches!(cli.cmd, Some(Command::Init)) {
        println!("prompter_precmd () {{");
        println!(
            "    eval \"$({})\"",
            std::env::current_exe().unwrap().display()
        );
        println!("}}");
        println!("autoload -Uz add-zsh-hook");
        println!("add-zsh-hook precmd prompter_precmd");
        println!("PS1=\"{}\"", config.left_prompt());
        println!("RPS1=\"{}\"", config.right_prompt());
    } else {
        for (i, v) in config.render().into_iter().enumerate().skip(1) {
            println!("psvar[{i}]={v:?}");
        }
    }
}
