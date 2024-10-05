mod parse_sigma;
use parse_sigma::sigma::SigmaRule;
fn main() {
    SigmaRule::parse_rule_from_file(
        "/home/debian/sigma/rules/windows/dns_query/dns_query_win_appinstaller.yml".to_string(),
    );
}
