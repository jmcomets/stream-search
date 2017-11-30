use std::env;
use std::io;
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

fn words<S: AsRef<str>>(s: S) -> Vec<String> {
    s.as_ref().split(' ').map(String::from).filter(|item| !item.is_empty()).collect()
}

fn main() {
    let arg = env::args().nth(1)
        .unwrap_or_else(|| exit!("usage: stream-search <needle>"));
    let needle = words(arg);
    println!("needle: {:?}", needle);

    let input = io::stdin();
    let reader = BufReader::new(input);

    // TODO: handle errors better
    let haystack = reader.lines().map(Result::unwrap).flat_map(words);

    let data = haystack.collect::<Vec<_>>();
    println!("data: {:?}", data);
    let haystack = data.into_iter();

    let mut haystack = haystack;

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
