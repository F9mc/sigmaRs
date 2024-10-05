mod parse_sigma;
use parse_sigma::sigma::{self, SigmaRule};

extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};

fn main() {
    let mut verbose = false;
    let mut recursion = false;
    let mut path = String::new();
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
        ap.refer(&mut recursion).add_option(
            &["-r", "--recursion"],
            StoreTrue,
            "Load all sigma files within a folder",
        );
        ap.parse_args_or_exit();
    }

    if path != String::new() {
        if recursion {
            let rules = SigmaRule::load_rule_from_folder(path);
            if verbose {
                for r in rules {
                    println!("{r}")
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
