use terminal_size::terminal_size;
use clap::{App, Arg, ArgMatches, SubCommand, AppSettings};

const APP_NAME: &str = crate_name!();
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let opt = Rad::from_args();
    let matches = get_matches();

    let res = match matches.subcommand_name() {
        Some("add") => subcommand::Add::run(&self),
        Some("add-env") => subcommand::AddEnv::run(&self),
        Some("list") => subcommand::List::run(&self),
};
}

fn get_matches<'a>() -> ArgMatches<'a> {
    App::new(APP_NAME)
        .set_term_width(terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(0))
        .version(CARGO_PKG_VERSION)
        .author(CARGO_PKG_AUTHORS)
        .about(concat!("Rust Aliasing Directory"))
        .global_setting(AppSettings::ColoredHelp)
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("add")
                .about("Add directory alias")
                .alias("a")
                .arg(Arg::with_name("ADD_VAR")
                    .value_name("ALIAS")
                    .index(1)
                    .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("add-env")
                .about("Add new environment variable for alias")
                .alias("ae")
                .arg(Arg::with_name("ADD_ENV_VAR")
                    .value_name("ENVIRONMENT_VAR")
                    .index(1)
                    .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("completions-alias")
                .about("List alias and environment variables for completions")
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize rualdi")
                    .arg(Arg::with_name("bash")
                        .long("bash")
                        .short("b")
                        .conflicts_with("zsh")
                        .help("Initialize bash"))
                    .arg(Arg::with_name("zsh")
                        .long("zsh")
                        .short("z")
                        .conflicts_with("bash")
                        .help("Initialize zsh"))
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List alias and environment variables")
                .alias("l")
        )
        .subcommand(
            SubCommand::with_name("list-env")
                .about("List environment variables only")
                .alias("le")
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove alias")
                .alias("r")
                .arg(Arg::with_name("REMOVE_VAR")
                    .value_name("ALIAS")
                    .index(1)
                    .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("remove-env")
                .about("Remove environment variable")
                .alias("re")
                .arg(Arg::with_name("REMOVE_ENV_VAR")
                    .value_name("ENVIRONMENT_VAR")
                    .index(1)
                    .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("resolve")
                .about("Resolve name of alias to directory")
                .arg(Arg::with_name("RESOLVE_VAR")
                    .value_name("alias")
                    .index(1)
                    .required(true)
                )
                .arg(Arg::with_name("RESOLVE_VAR").value_name("alias").index(1).required(true))
        )
        .subcommand(
            SubCommand::with_name("resolve-env")
                .about("Resolve name of environment variable to alias")
                .arg(Arg::with_name("RESOLVE_ENV_VAR").value_name("RESOLVE_ENV_VAR").index(1).required(true))
        )
        .get_matches()
}

/////////////////////////////////////

for variant in Shell::variants()
    .iter()
    .filter_map(|v| Shell::from_str(v).ok())
{
    app.gen_completions(env!("CARGO_PKG_NAME"), variant, &out_dir);
}

/////////////////////////////////////

format!("Comletion scripts have been generated in \"{}\"",
    out_dir.green().bold()
)

          for variant in Shell::variants()
            .iter()
            .filter_map(|v| Shell::from_str(v).ok())
            .zip_longest(
                CompType::variants()
                    .iter()
            )
        {
            match variant {
                Both(x, y) => {
                    variants.push(x);
                    variants.push(y);
                },
                Left(x) => variants.push(x),
                Right(y) => variants.push(y),
            }
        }

// ##############################
// pub enum Error<'src> {
//     Internal {
//         message: String,
//     },
// }

// pub type RunResult<'a, T> = Result<T, Error<'a>>;
//
// impl<'src> Error<'src> {
//     pub fn internal(message: impl Into<String>) -> Self {
//         Self::Internal {
//             message: message.into(),
//         }
//     }
// }
//
// match self {
//     Internal { message } => {
//         write!(
//           f,
//           "test",
//           message
//         )?;
//     }
// }
// ##############################
