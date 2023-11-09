use args::Args;
use clap::Parser;
use std::{error::Error, fs::read_to_string};

mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file_contents = read_to_string(args.filename)?;
    let doc = roxmltree::Document::parse(&file_contents)?;
    let paths = doc
        .descendants()
        .filter(|x| x.tag_name().name() == "path")
        .collect::<Vec<_>>();

    let position_string_vec = paths
        .iter()
        .flat_map(|x| x.attribute("d"))
        .collect::<Vec<_>>();

    Ok(())
}

enum State {
    End,
    MoveTo,
    DrawLine,
}

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

    fn parse_path(x: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut state: State = State::End;
        Ok(x.split(" ").flat_map(|t| match t {
            "M" => {
                state = State::MoveTo;
                None
            }
            t => {
                match t
                    .split_once(',')
                    .map(|x| (x.0.parse::<f64>()?, x.1.parse::<f64>()?))?
                {
                    (x, y) => Self::from_option(&state, x, y)
                }
                }
            }
        ).collect())
    }

    fn from_option(state: &State, x: f64, y: f64) -> Option<Self>{
        let x = x;
        let y = y;
        match state{
            State::End => None,
            State::MoveTo => Some(Self::MoveTo(x.floor() as u8, y.floor() as u8)),
            State::DrawLine => Some(Self::DrawLine(x.floor() as u8, y.floor() as u8)),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_line() {}
}
