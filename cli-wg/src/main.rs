use std::path;
use std::fs;
use std::io;

use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;

use cli_wg::find_matches;

#[derive(StructOpt, Debug)]
struct Cli {
  #[structopt(required = true)]
  pattern: String,
  // #[structopt(parse(from_os_str), short = "p", long = "path")]
  #[structopt(parse(from_os_str))]
  path: path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
  let args = Cli::from_args();
  let content = fs::read_to_string(&args.path)
    .with_context(|_| format!("Error reading `{}`", &args.path.display()))?;

  find_matches(&content, &args.pattern, &mut io::stdout())?;

  Ok(())
}
