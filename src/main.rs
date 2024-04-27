mod types;
mod variables;
mod consts;
mod strings;
mod std;
mod functions;
use consts::SECONDS_IN_HOUR;

fn main() {
    variables::mod_variables::variables();
    types::my_module::types();
    strings::module_strings::strings();
    std::module_s::s();
    functions::module_fn::function();
}
