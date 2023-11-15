use args::Args;
use clap::Parser;
use std::{error::Error, fs::read_to_string, path::Path};

mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let instructions = Instrcution::from_file(&args.filename)?;

    println!("{:?}", instructions);

    Ok(())
}

fn get_paths_from_file(filename: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let file_contents = read_to_string(filename)?;
    let doc = roxmltree::Document::parse(&file_contents)?;
    let paths = doc
        .descendants()
        .filter(|x| x.tag_name().name() == "path")
        .collect::<Vec<_>>();

    Ok(paths
        .iter()
        .flat_map(|x| x.attribute("d"))
        .map(|x| x.to_owned())
        .collect::<Vec<String>>())
}

enum State {
    MoveTo,
    DrawLine,
}

#[derive(Debug)]
enum Instrcution {
    EOF,
    MoveTo(u8, u8),
    DrawLine(u8, u8),
}

impl Instrcution {
    fn to_bytes(&self) -> [u8; 3] {
        match self {
            Instrcution::EOF => [0, 0, 0],
            Instrcution::MoveTo(x, y) => [1, *x, *y],
            Instrcution::DrawLine(x, y) => [2, *x, *y],
        }
    }

    fn from_file(filename: &Path) -> Result<Vec<Instrcution>, Box<dyn Error>>{
        let paths = get_paths_from_file(filename)?;
        let instructions: Vec<Instrcution> = paths
                .iter()
                .map(|x| Instrcution::parse_path(x))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect();
        Ok(instructions)
    }

    fn parse_path(x: &String) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut state: State = State::MoveTo;
        Ok(x.split(" ")
            .flat_map(|t| match t {
                "M" => {
                    state = State::MoveTo;
                    None
                }
                "Z" => {
                    None
                }
                t => {
                    match t
                        .split_once(',')
                        .map(|x| {
                            (
                                x.0.parse::<f64>()
                                    .expect(format!("Could not parse {}", x.0).as_str()),
                                x.1.parse::<f64>()
                                    .expect(format!("Could not parse {}", x.1).as_str()),
                            )
                        })
                        .expect(format!("Did not find a cordinate tuple at {}", t).as_str())
                    {
                        (x, y) => Self::from_option(&mut state, x, y),
                    }
                }
            })
            .collect())
    }

    fn from_option(state: &mut State, x: f64, y: f64) -> Option<Self> {
        let x = x;
        let y = y;
        match state {
            State::MoveTo => {
                // This horrendous mutable code is a result of how .svg files use implicit instrcution prefixes
                *state = State::DrawLine;
                Some(Self::MoveTo(x.floor() as u8, y.floor() as u8))
            }
            State::DrawLine => Some(Self::DrawLine(x.floor() as u8, y.floor() as u8)),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_line() {}
}
