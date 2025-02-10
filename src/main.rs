use clap::{Arg, ArgAction, Command};
use clap_stdin::FileOrStdin;
use graphql_normalize::normalize;

fn main() {
    let command = Command::new("graphql-normalize")
        .arg(
            Arg::new("path")
                .default_value("-")
                .required(false)
                .value_parser(clap::value_parser!(FileOrStdin)),
        )
        .arg(Arg::new("minify").short('m').action(ArgAction::SetTrue))
        .arg_required_else_help(false);

    let matches = command.get_matches();

    let path = matches.get_one::<FileOrStdin<String>>("path").unwrap();

    let query_content: String = path.clone().contents().expect("Unable to read input");
    let normalized = normalize(&query_content).expect("Could not normalize");

    if matches.get_flag("minify") {
        let minified = graphql_parser::minify_query(normalized).expect("Could not minify");

        println!("{}", minified);
    } else {
        println!("{}", normalized);
    }
}
