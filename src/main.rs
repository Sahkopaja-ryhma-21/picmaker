use std::{fs::read_to_string, error::Error};
use args::Args;
use clap::Parser;

mod args;

fn main() -> Result<(), Box<dyn Error>>{
    let args = Args::parse();
    let file_contents = read_to_string(args.filename)?;
    let doc = roxmltree::Document::parse(&file_contents)?;
    let paths = doc.descendants().filter(|x| x.tag_name().name()== "path").collect::<Vec<_>>();

    let position_vec = paths.iter().map(|x| x.attribute("d")).collect::<Vec<_>>();

    println!("{:?}", position_vec);

    Ok(())
}

