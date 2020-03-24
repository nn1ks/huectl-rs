use structopt::StructOpt;

/// Prints capabilities of resources
#[derive(Debug, StructOpt)]
pub struct Get {
    /// Prints the output in JSON format
    #[structopt(long, short)]
    pub json: bool,
}
