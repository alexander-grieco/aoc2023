use std::{collections::HashMap, fs};

fn get_input() -> &'static str {
    "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
}

fn process_line(line: &str) -> (Vec<u64>, &[u8]) {
    let (symbols, arrangement) = line.split_once(" ").unwrap();
    let arr = arrangement
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    (arr, symbols.as_bytes())
}

fn num_possible(
    cache: &mut HashMap<(u64, u64, u64), u64>,
    symbols: &[u8],
    arr: &[u64],
    prev: Option<u64>,
) -> u64 {
    // If we've gone through the string return values
    if symbols.is_empty() {
        return match (prev, arr.len()) {
            (None, 0) => 1, // We are not in a pattern and no more numbers left in pattern
            (Some(x), 1) if x == arr[0] => 1, // We are in a pattern but it's valid
            _ => 0,         // not a possible arrangement
        };
    }

    // in a pattern but no more numbers left
    if prev.is_some() && arr.is_empty() {
        return 0;
    }

    // If current position exists in cache, return it's value (I think this is memoization?)
    let key = (symbols.len() as u64, prev.unwrap_or(0), arr.len() as u64);
    if let Some(&val) = cache.get(&key) {
        return val;
    }

    let possible = match (symbols[0], prev) {
        // end of pattern, doesn't match array
        (b'.', Some(x)) if x != arr[0] => 0,
        // End of pattern, matches array
        (b'.', Some(_)) => num_possible(cache, &symbols[1..], &arr[1..], None),
        // Continuing non-pattern
        (b'.', None) => num_possible(cache, &symbols[1..], arr, None),
        // Within pattern, continue it
        (b'#', Some(_)) => num_possible(cache, &symbols[1..], arr, prev.map(|x| x + 1)),
        // Start of pattern
        (b'#', None) => num_possible(cache, &symbols[1..], arr, Some(1)),
        // Wildcard, within pattern
        (b'?', Some(x)) => {
            // this checks for possibles answers by continuing the pattern
            let mut ans = num_possible(cache, &symbols[1..], arr, prev.map(|x| x + 1));
            // If x is in arr, check for cases where '?' is '.' (ie end the pattern)
            if x == arr[0] {
                ans += num_possible(cache, &symbols[1..], &arr[1..], None);
            }
            ans
        }
        // Wildcard, not in pattern, check for both cases: start a pattern, continue non-pattern
        (b'?', None) => {
            num_possible(cache, &symbols[1..], arr, None)
                + num_possible(cache, &symbols[1..], arr, Some(1))
        }
        // should never get here
        _ => panic!("Invalid symbol"),
    };
    cache.insert(key, possible);
    possible
}

fn main() {
    //let input = get_input();
    let input = fs::read_to_string("inputs/p12.txt").unwrap();

    let mut mem = HashMap::new();
    let sump1 = input.lines().fold(0, |acc, line| {
        let (arr, symbols) = process_line(line);
        mem.clear();
        let ret = num_possible(&mut mem, symbols, &arr, None);
        acc + ret
    });
    println!("p1: {}", sump1);

    let sump2 = input.lines().fold(0, |acc, line| {
        let (arr, symbols) = process_line(line);
        let symbols = std::iter::repeat(symbols)
            .take(5)
            .fold(Vec::new(), |mut acc, x| {
                acc.extend_from_slice(x);
                acc.extend_from_slice(&[b'?']);
                acc
            });
        let arr = (0..5).flat_map(|_| &arr).copied().collect::<Vec<_>>();
        mem.clear();

        // need the minus 1 because I couldn't figure out a way to copy the byte array 5 times
        // without having a trailing '?' :-(
        let ret = num_possible(&mut mem, &symbols[..symbols.len() - 1], &arr, None);
        acc + ret
    });
    println!("p2: {}", sump2);
}
