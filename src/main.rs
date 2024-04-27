mod types;
mod variables;
mod consts;
mod strings;
use consts::SECONDS_IN_HOUR;

fn main() {
    variables::mod_variables::variables();
    types::my_module::types();
    strings::module_strings::strings();
}
