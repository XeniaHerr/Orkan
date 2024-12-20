use clap::{Parser, Args, Subcommand};
use super::color::Color;


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {

    #[arg(short, long, global = true, default_value = "Mononoki Nerd Font")]
   pub font:  String,

    #[arg(short, long, value_parser = clap::value_parser!(i32), default_value_t = 0, global = true)]
    pub top_margin: i32,

    #[arg(short, long, default_value_t = 0, value_parser = clap::value_parser!(i32), global = true)]
    pub left_margin: i32,

    #[arg(long, value_parser = Color::from_hex_string, global = true, default_value = "000000") ]
    pub fontcolor: Color,

    #[arg(long, value_parser = Color::from_hex_string, global = true, default_value = "ffffff") ]
    pub backgroundcolor: Color,

    #[arg(long, value_parser = Color::from_hex_string, global = true, default_value = "ff0000") ]
    pub highlight: Color,

    #[arg(long, global = true, default_value_t = 20)]
    pub height : i32,

    #[command(subcommand)]
     pub subcommand: Option<SubCommand>,
}


#[derive(Debug,Subcommand)]
pub enum SubCommand {
    /// Select from binaries
    Run,
    /// select from custom Input
    #[command(name = "custom")]
    Custom(CustomCommandArgs),

}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct CustomCommandArgs {
    /// Read from file
    #[arg(long, name = "from-file")]
   pub from_file: Option<String>,

    /// Read from stdin
    #[arg(long, name = "from-stdin")]
    pub from_stdin: bool,
}

