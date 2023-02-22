use pgn_reader::{BufferedReader, SanPlus, Skip, Visitor};

const MAX_PLIES: usize = 80;
const PRINT: bool = false;
const DRAWS: bool = false;
const ALL: bool = true;

fn main() {
    let variant = std::env::args().nth(1).unwrap();
    println!("{variant}");

    let file = std::fs::File::open(format!("data/lichess_db_{variant}_rated_2023-01.pgn")).unwrap();
    let mut reader = BufferedReader::new(file);

    let mut visitor = Stats::new();
    if ALL {
        reader.read_all(&mut visitor).unwrap();
    } else {
        let limit = if PRINT { 30_000 } else { 100_000 };
        for i in 0..limit {
            if i % 10_000 == 0 {
                println!("{i}");
            }
            if reader.read_game(&mut visitor).unwrap().is_none() {
                break;
            }
        }
    }

    let total = visitor.end.iter().sum::<u64>();
    let max = visitor
        .end
        .chunks(2)
        .map(|s| s.iter().sum::<u64>())
        .max()
        .unwrap();
    let mut cum = 0;
    for i in 0..visitor.end.len() / 2 {
        let val = visitor.end[2 * i] + visitor.end[2 * i + 1];
        cum += val;
        let cump = cum * 100 / total;
        let width = (val * 300 / max) as usize;
        println!("{i:2}: {val:4} {cump:3}% {:#<width$}", "");
    }
    // for (i, v) in visitor.end.iter().enumerate() {
    //     println!("{i:2}: {v:4}");
    // }
}

struct Stats {
    end: [u64; MAX_PLIES],
    moves: u32,
    skip: bool,
    url: Vec<u8>,
}

impl Stats {
    fn new() -> Self {
        Self {
            end: [0; MAX_PLIES],
            moves: 0,
            skip: false,
            url: Vec::new(),
        }
    }
}

impl Visitor for Stats {
    type Result = ();

    fn end_game(&mut self) {
        if !self.skip {
            if self.moves as usize >= self.end.len() {
                self.end[self.end.len() - 1] += 1;
            } else {
                self.end[self.moves as usize] += 1;
            }
            if PRINT && self.moves < 60 && self.moves > 40 {
                println!(
                    "{:2} {}",
                    self.moves,
                    std::str::from_utf8(&self.url).unwrap()
                );
            }
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn san(&mut self, _san: SanPlus) {
        self.moves += 1;
    }

    fn begin_game(&mut self) {
        self.skip = false;
        self.moves = 0;
    }

    fn header(&mut self, key: &[u8], value: pgn_reader::RawHeader<'_>) {
        if !DRAWS {
            return;
        }
        if key == b"Result" {
            if value.as_bytes() != b"1/2-1/2" {
                self.skip = true;
            }
        } else if key == b"BlackElo" || key == b"WhiteElo" {
            if value.as_bytes().len() < 4 || value.as_bytes() < b"1701" {
                self.skip = true;
            }
        } else if PRINT && key == b"Site" {
            value.0.clone_into(&mut self.url);
        }
    }

    fn end_headers(&mut self) -> Skip {
        Skip(self.skip)
    }
}
