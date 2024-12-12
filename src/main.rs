use clap::Parser;
use cli::Command;

mod cli;
mod config;
mod sources;

fn main() {
    let config = config::Config::default();
    let cli = cli::Cli::parse();
    if matches!(cli.cmd, Some(Command::Init)) {
        println!(
            r#"
__prompter_precmd () {{
    eval "$({exe})"
}}

autoload -Uz add-zsh hook
add-zsh-hook precmd __prompter_precmd

PS1="{left_prompt}"
PS2="{right_prompt}"
"#,
            exe = std::env::current_exe().unwrap().display(),
            left_prompt = config.left_prompt(),
            right_prompt = config.right_prompt()
        );
    } else {
        for (i, v) in config.render().into_iter().enumerate().skip(1) {
            println!("psvar[{i}]={v:?}");
        }
    }
}
