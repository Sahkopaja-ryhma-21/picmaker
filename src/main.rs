use args::Args;
use clap::Parser;
use point::Point;
use roxmltree::Document;
use std::{error::Error, fs::read_to_string, path::Path};

mod args;
mod point;
mod serial;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let instructions = Instrcution::from_file(&args.filename)?;
    serial::upload_data(
        instructions
            .iter()
            .flat_map(Instrcution::to_bytes)
            .collect(),
        args.baud_rate,
    )?;

    Ok(())
}

fn get_paths_from_file(doc: &Document) -> Result<Vec<String>, Box<dyn Error>> {
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

/// Parses viewbox dimension from a string value
fn parse_viewbox(x: &str) -> (f64, f64) {
    fn helper(s: &str) -> f64 {
        s.parse::<f64>().expect("svg viewbox not parsable")
    }

    let binding = x.replace(",", " ");
    let mut i = binding.split(" ").map(|s| s.trim());
    let x1: f64 = helper(i.next().expect("x1 not found"));
    let y1: f64 = helper(i.next().expect("y1 not found"));
    let x2: f64 = helper(i.next().expect("x2 not found"));
    let y2: f64 = helper(i.next().expect("y2 not found"));
    (x2 - x1, y2 - y1)
}

enum State {
    MoveTo,
    DrawLine,
    ///Relative moveto
    MoveToR,
    ///Relative drawline
    DrawLineR,
    DrawVertical,
    DrawVerticalR,
    DrawHorizontal,
    DrawHorizontalR,
}

#[derive(Debug, Clone)]
enum Instrcution {
    EOF,
    MoveTo(Point),
    DrawLine(Point),
}

impl Instrcution {
    fn to_bytes(&self) -> [u8; 3] {
        match self {
            Instrcution::EOF => [0, 0, 0],
            Instrcution::MoveTo(p) => [1, p.0, p.1],
            Instrcution::DrawLine(p) => [2, p.0, p.1],
        }
    }

    fn from_file(filename: &Path) -> Result<Vec<Instrcution>, Box<dyn Error>> {
        let file_contents = read_to_string(filename)?;
        let doc = roxmltree::Document::parse(&file_contents)?;
        let paths = get_paths_from_file(&doc)?;
        let (width, height) = parse_viewbox(
            doc.root_element()
                .attribute("viewBox")
                .expect("SVG does not have a viewbox size"),
        );
        let mut instructions: Vec<Instrcution> = paths
            .iter()
            .map(|x| Instrcution::parse_path(x, width, height))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        instructions.push(Instrcution::EOF);
        Ok(instructions)
    }

    fn parse_path(x: &String, width: f64, height: f64) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut state: State = State::MoveTo;
        let mut start_point: Option<Point> = None;
        let mut current_point: Option<Point> = None;
        Ok(x.split(" ")
            .flat_map(|t| match t {
                "M" => {
                    state = State::MoveTo;
                    None
                }
                "m" => {
                    state = State::MoveToR;
                    None
                }
                "Z" | "z" => Some(Self::DrawLine(
                    start_point.expect("Start position to defined").clone(),
                )),
                "L" => {
                    state = State::DrawLine;
                    None
                }
                "l" => {
                    state = State::DrawLineR;
                    None
                }
                "H" => {
                    state = State::DrawHorizontal;
                    None
                }
                "h" => {
                    state = State::DrawHorizontalR;
                    None
                }
                "V" => {
                    state = State::DrawVertical;
                    None
                }
                "v" => {
                    state = State::DrawVerticalR;
                    None
                }
                t => {
                    match t
                        .split_once(',')
                        .map(|x| {
                            (
                                255f64
                                    * x.0
                                        .parse::<f64>()
                                        .expect(format!("Could not parse {}", x.0).as_str())
                                    / width,
                                255f64
                                    * x.1
                                        .parse::<f64>()
                                        // It is possible for there to be no second cordinate
                                        .unwrap_or(f64::NAN)
                                    / height,
                            )
                        }).unwrap_or_else(||(255f64*t.parse::<f64>().expect("Failed")/height, 0f64))
                    {
                        (x, y) => {
                            let inst = Self::create(&mut state, x, y, current_point.as_ref());
                            match inst {
                                Instrcution::EOF => (),
                                Instrcution::MoveTo(p) | Instrcution::DrawLine(p) => {
                                    if start_point.is_none() {
                                        start_point = Some(p)
                                    };
                                    current_point = Some(p);
                                }
                            }
                            Some(inst)
                        }
                    }
                }
            })
            .collect())
    }

    fn create(state: &mut State, x: f64, y: f64, lp: Option<&Point>) -> Self {
        // x is the first parameter. It is the xcordinate, execept for vertical lines, where x is
        // used as the horisontal cordinate aswell.

        let x = x.floor() as i16;
        let y = y.floor() as i16;
        let xdiff: Option<u8> = lp.map(|v| (x + (v.0 as i16)).try_into().unwrap_or(u8::MAX));
        let ydiff: Option<u8> = lp.map(|v| (y + (v.1 as i16)).try_into().unwrap_or(u8::MAX));
        let xa: u8 = x.try_into().unwrap_or(u8::MAX);
        let ya: u8 = y.try_into().unwrap_or(u8::MAX);
        match state {
            State::MoveTo => {
                // This horrendous mutable code is a result of how .svg files use implicit instrcution prefixes
                *state = State::DrawLine;
                Self::MoveTo(Point(xa, ya))
            }
            State::DrawLine => Self::DrawLine(Point(xa, ya)),
            State::MoveToR => {
                *state = State::DrawLineR;
                let p = match lp {
                    Some(_) => Point(xdiff.unwrap(), ydiff.unwrap()),
                    None => Point(xa, ya),
                };
                Self::MoveTo(p)
            }
            State::DrawLineR => Self::DrawLine(Point(
                xdiff.expect("Relative line without start point"),
                ydiff.expect("Relative line wihout start point"),
            )),
            State::DrawVertical => {
                Self::DrawLine(Point(lp.expect("Vertical line from nothing").0, xa))
            }
            State::DrawVerticalR => Self::DrawLine(Point(
                lp.expect("Vertical line from nothing").0,
                xdiff.expect("Relative without startpoint"),
            )),
            State::DrawHorizontal => {
                Self::DrawLine(Point(xa, lp.expect("Horizontal line from nothing").1))
            }
            State::DrawHorizontalR => Self::DrawLine(Point(
                xdiff.expect("Relative without startpoint"),
                lp.expect("Horizontal line from nothing").1,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::Instrcution;

    #[test]
    fn test_line() {
        let res: Vec<Vec<u8>> = vec![vec![1, 0, 0], vec![2, 255, 255], vec![0, 0, 0]];
        let i = Instrcution::from_file(
            &PathBuf::from_str("test.svg").expect("Did not find file test.svg"),
        )
        .unwrap();
        i.iter()
            .zip(res)
            .for_each(|x| assert_eq!(x.1, x.0.to_bytes()));
    }

    #[test]
    fn test_multiple_lines() {
        let res: Vec<u8> = vec![
            1, 0, 0, 2, 255, 255, 1, 216, 137, 2, 83, 180, 2, 34, 111, 2, 140, 49, 2, 166, 108, 2,
            237, 89, 2, 216, 137, 0, 0, 0,
        ];
        let i = Instrcution::from_file(
            &PathBuf::from_str("test2.svg").expect("Did not find file test2.svg"),
        )
        .unwrap();
        i.iter()
            .flat_map(Instrcution::to_bytes)
            .zip(res)
            .for_each(|(x, correct)| assert_eq!(x, correct));
    }
    #[test]
    #[ignore = "Used to print"]
    fn test_print() {
        let i = Instrcution::from_file(
            &PathBuf::from_str("Aalto-iso.svg").expect("Did not find file aalto.svg"),
        );
        i.unwrap()
            .iter()
            .flat_map(Instrcution::to_bytes)
            .for_each(|x| println!("{x}"));
    }
}
