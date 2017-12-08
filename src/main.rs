use std::mem;
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
    s.as_ref().split(' ')
        .map(|s| s.trim_matches(|c| !char::is_alphabetic(c)))
        .map(String::from).filter(|item| !item.is_empty()).collect()
}

fn sum_adjacent_differences<'a, It>(iter: It) -> usize
    where It: Iterator<Item=&'a usize> + Clone
{
    let cloned_iter = iter.clone();
    iter.skip(1).zip(cloned_iter)
        .map(|(i2, i1)| i2 - i1)
        .sum()
}

fn main() {
    let arg = env::args().nth(1)
        .unwrap_or_else(|| exit!("usage: stream-search <needle>"));
    let needle = words(arg);
    //println!("needle: {:?}", needle);

    let input = io::stdin();
    let reader = BufReader::new(input);

    // TODO: handle errors better
    let haystack = reader.lines().map(Result::unwrap).flat_map(words);

    //let data: Vec<_> = haystack.collect();
    //println!("data: {:?}", data);
    //let haystack = data.into_iter();

    let mut haystack = haystack.enumerate();

    let mut best = None;
    let mut partial_matches: Vec<Vec<Vec<usize>>> = vec![vec![]; needle.len()];

    while let Some((cursor, word)) = haystack.next() {
        let matched_needle_indices: Vec<_> = needle.iter().enumerate()
            .filter(|&(_, w)| w == &word)
            .map(|(i, _)| i)
            .collect();

        // step 1: expand each partial solution
        for &i in matched_needle_indices.iter() {
            // default-initialization
            if i == 0 {
                partial_matches[0].push(vec![]);
            }

            for partial_match in partial_matches[i].iter_mut() {
                partial_match.push(cursor);
            }
        }

        // step 2: move each expanded partial solution right
        //  > on overflow, the solution is full and should be compared to the best solution
        //
        // Note that this should be run in reverse order, as we're shifting elements right.
        for i in matched_needle_indices.into_iter().rev() {
            let mut extended_matches = vec![];
            mem::swap(&mut extended_matches, &mut partial_matches[i]);

            if i + 1 < partial_matches.len() {
                partial_matches[i + 1].extend(extended_matches);
            } else {
                for indices in extended_matches {
                    let new_score = sum_adjacent_differences(indices.iter());

                    //print!("match = {:?}; score = {:?} ... ", indices, new_score);

                    let is_better = best.as_ref()
                        .map(|&(score, _)| new_score < score)
                        .unwrap_or(true);
                    if is_better {
                        //println!("better");
                        best = Some((new_score, indices));
                    } else {
                        //println!("worse");
                    }
                }
            }
        }
    }

    if let Some((_, indices)) = best {
        println!("Best match: {:?}", indices);
    } else {
        println!("No match");
    }
}
