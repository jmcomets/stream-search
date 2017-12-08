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

    let data: Vec<_> = haystack.collect();
    println!("data: {:?}", data);
    let haystack = data.into_iter();

    let mut haystack = haystack.enumerate();

    let mut best = None;
    let mut partial_matches: Vec<Vec<(Option<usize>, usize)>> = vec![vec![/* this one can be huge */]; needle.len()];

    while let Some((cursor, word)) = haystack.next() {
        let matched_needle_indices: Vec<_> = needle.iter().enumerate()
            .filter(|&(_, w)| w == &word)
            .map(|(i, _)| i)
            .collect();

        // step 1: expand each partial solution
        for &i in matched_needle_indices.iter() {
            // default-initialization
            if i == 0 {
                partial_matches[0].push((None, 0));
            }

            for &mut (ref mut prev_cursor, ref mut cost) in partial_matches[i].iter_mut() {
                *cost += cursor - prev_cursor.unwrap_or(cursor);
                *prev_cursor = Some(cursor);
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
                for (_, new_cost) in extended_matches {
                    print!("cost = {:?} ... ", new_cost);
                    //print!("match = {:?}; cost = {:?} ... ", indices, new_cost);

                    let is_better = best.as_ref()
                        .map(|&cost| new_cost < cost)
                        .unwrap_or(true);
                    if is_better {
                        println!("better");
                        best = Some(new_cost);
                    } else {
                        println!("worse");
                    }
                }
            }
        }
    }

    if let Some(cost) = best {
        println!("Best match: {:?}", cost);
    } else {
        println!("No match");
    }
}
