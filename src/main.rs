// start -> end
// inclusive
struct Range(u32, u32);

struct Pair {
    first: Range,
    second: Range,
}
impl Pair {
    fn is_complete_overlap(&self) -> bool {
        if self.first.0 <= self.second.0 && self.first.1 >= self.second.1 {
            return true;
        }

        if self.second.0 <= self.first.0 && self.second.1 >= self.first.1 {
            return true;
        }

        false
    }
}

fn create_pairs_from_input() -> Vec<Pair> {
    let input = include_str!("../input.txt");

    let mut pairs = vec![];
    for line in input.lines() {
        // comma separated, get the first and second
        let mut split = line.split(',');
        let first_range = split.next().unwrap();
        let mut range_split = first_range.split('-');
        let x = range_split.next().unwrap();
        let y = range_split.next().unwrap();

        let second_range = split.next().unwrap();
        let mut range_split = second_range.split('-');
        let a = range_split.next().unwrap();
        let b = range_split.next().unwrap();

        let x = x.parse::<u32>().unwrap();
        let y = y.parse::<u32>().unwrap();
        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();

        let pair = Pair {
            first: Range(x, y),
            second: Range(a, b),
        };
        pairs.push(pair);
    }

    pairs
}

fn get_total_overlapping(pairs: Vec<Pair>) -> u32 {
    let mut total: u32 = 0;
    for pair in pairs.into_iter() {
        if pair.is_complete_overlap() {
            total += 1;
        }
    }

    total
}

fn main() {
    println!("Hello, world!");

    let pairs = create_pairs_from_input();
    let total = get_total_overlapping(pairs);
    println!("total : {:}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    // part 1
    #[test]
    fn test_get_completely_overlapping_pairs() {
        let pairs = vec![
            Pair {
                first: Range(2, 4),
                second: Range(6, 8),
            },
            Pair {
                first: Range(2, 3),
                second: Range(4, 5),
            },
            Pair {
                first: Range(5, 7),
                second: Range(7, 9),
            },
            Pair {
                first: Range(2, 8),
                second: Range(3, 7),
            },
            Pair {
                first: Range(6, 6),
                second: Range(4, 6),
            },
            Pair {
                first: Range(2, 6),
                second: Range(4, 8),
            },
        ];

        assert_eq!(get_total_overlapping(pairs), 2);
    }
}
