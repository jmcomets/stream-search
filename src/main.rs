use std::env;
use std::io;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::process;

macro_rules! exit {
    ($msg:expr) => {
        {
            eprintln!($msg);
            process::exit(1);
        }
    }
}

fn is_byte_whitespace(x: u8) -> bool {
    const WHITESPACE: &[u8] = &[b' ', b'\t', b'\r', b'\n'];
    WHITESPACE.iter().any(|&y| x == y)
}

fn main() {
    let needle: Vec<_> = env::args().nth(1)
        .unwrap_or_else(|| exit!("usage: stream-search <needle>"))
        .split(' ')
        .map(|s| s.as_bytes().to_owned())
        .collect();
    println!("needle: {:?}", needle);

    let input = io::stdin();
    let mut reader = BufReader::new(input);

    // TODO: handle errors better
    let mut data = vec![];
    let _ = reader.read_to_end(&mut data).unwrap();
    let mut haystack = data.split(|&c| c == b' ').map(ToOwned::to_owned);
    println!("data: {:?}", data);
    //let mut haystack = reader.split(b' ').map(Result::unwrap);
    let mut haystack = haystack.map(|item| item.into_iter().filter(|&x| is_byte_whitespace(x)).collect::<Vec<_>>());

    let mut cursor = 0;
    let mut lookahead = vec![];

    let mut best = None;

    'outer: loop {
        cursor -= lookahead.len();
        let mut next_lookahead = vec![];

        let mut i = 0;
        let mut indices = vec![];

        while indices.len() < needle.len() {
            let item = lookahead.pop()
                .or_else(|| {
                    let item = haystack.next();

                    if let Some(item) = item.as_ref() {
                        if !indices.is_empty() {
                            next_lookahead.push(item.clone());
                        }
                    }

                    item
                });

            if let Some(item) = item {
                cursor += 1;

                if item == needle[i] {
                    indices.push(cursor - 1);
                    i += 1;
                }
            } else {
                break 'outer;
            }
        }

        let new_score: usize = indices.iter().skip(1)
            .zip(indices.iter())
            .map(|(i2, i1)| i2 - i1)
            .sum();

        let is_better = best.as_ref()
            .map(|&(score, _)| new_score < score)
            .unwrap_or(true);
        if is_better {
            best = Some((new_score, indices));
        }

        lookahead = next_lookahead;
    }

    if let Some((_, indices)) = best {
        println!("Best match: {:?}", indices);
    } else {
        println!("No match");
    }
}
