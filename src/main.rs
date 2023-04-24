pub mod libs;
use libs::exec::exec;
use libs::get_input::get_input;
use libs::parse_config::parse_config;
use libs::prompt::prompt;

fn main() {
    parse_config();
    loop {
        prompt();
        let userinput = get_input();
        exec(userinput);
    }
}
