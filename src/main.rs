extern crate argparse;
mod custom_error;
mod sentinel;
mod sigma;
use argparse::{ArgumentParser, Store, StoreTrue};
use sentinel::SentinelLogSource;
use sigma::SigmaRule;

fn main() {
    let mut verbose = false;
    let mut recursion = false;
    let mut path: String = String::new();
    let mut log_source = String::new();
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("A Rust tool to manipulate SigmaHQ rules");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut path).add_option(
            &["-p", "--path"],
            Store,
            "Path of the sigma file to manipulate",
        );
        ap.refer(&mut log_source).add_option(
            &["-l", "--log-sources"],
            Store,
            "Path of the custom log sources",
        );
        ap.refer(&mut recursion).add_option(
            &["-r", "--recursion"],
            StoreTrue,
            "Load all sigma files within a folder",
        );
        ap.parse_args_or_exit();
    }

    let log_sources = SentinelLogSource::load_sources(log_source);

    if path != String::new() {
        if recursion {
            let rules = SigmaRule::load_rule_from_folder(path);
            if verbose {
                for r in rules {
                    println!("{}", r.logsource)
                }
            }
        } else {
            let rule = match SigmaRule::parse_rule_from_file(path) {
                Ok(val) => val,
                Err(_) => panic!("Error reading the rule file"),
            };

            if verbose {
                println!("Working with '{rule}'");
            }
        }
    }
}
