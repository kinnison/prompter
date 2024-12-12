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
    print -P -f "\033]0;%s\033\\" "$__prompter_title"
}}

__prompter_preexec () {{
    local title
    print -v title -P -f "%s" "$__prompter_title"
    print -f "\033]0;%s 🏃‍♂️ %s\033\\" "$title" "$1"
}}

autoload -Uz add-zsh hook
add-zsh-hook precmd __prompter_precmd
add-zsh-hook preexec __prompter_preexec

PS1="{left_prompt}"
RPS1="{right_prompt}"
__prompter_title="{title}"
"#,
            exe = std::env::current_exe().unwrap().display(),
            left_prompt = config.left_prompt(),
            right_prompt = config.right_prompt(),
            title = config.title(),
        );
    } else {
        for (i, v) in config.render().into_iter().enumerate().skip(1) {
            println!("psvar[{i}]={v:?}");
        }
    }
}
