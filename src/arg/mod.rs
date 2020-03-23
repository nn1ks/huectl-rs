pub mod subcommand;
pub mod value;

use std::fmt;
use structopt::StructOpt;

/// A command line interface to Philips Hue
#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub subcommand: subcommand::Subcommand,
}

#[derive(Clone, Debug)]
pub struct ParseError {
    description: String,
}

impl ParseError {
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_owned(),
        }
    }

    pub fn from_integer_value<T: fmt::Display>(max_value: &T) -> Self {
        Self::new(&format!(
            "The value must be an integer between 0 and {} and can have '-' or '+' as prefix.",
            max_value
        ))
    }

    pub fn from_float_value<T: fmt::Display>(max_value: &T) -> Self {
        Self::new(&format!(
            "The value must be a floating point number between 0 and {} and can have '-' or '+' as prefix.",
            max_value
        ))
    }
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description)
    }
}
