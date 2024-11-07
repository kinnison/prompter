mod config;
mod sources;

fn main() {
    let config = config::Config::default();
    if std::env::args().nth(1).is_some() {
        println!("PS1=\"{}\"", config.left_prompt());
        println!("RPS1=\"{}\"", config.right_prompt());
    } else {
        for (i, v) in config.render().into_iter().enumerate().skip(1) {
            println!("psvar[{i}]={v:?}");
        }
    }
}
