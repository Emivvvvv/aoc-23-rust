use std::fs;

fn main() {
    let input = read_file_string("src/input.txt").unwrap();
    let lines_vector = input.lines().collect::<Vec<_>>();

    #[derive(Debug, PartialEq, PartialOrd)]
    enum Type {
        FiveOfAKind,
        FourOfAKind,
        FullHouse,
        ThreeOfAKind,
        TwoPair,
        OnePair,
        HighCard,
    }

    #[derive(Debug)]
    struct Hand {
        cards: String,
        card_type: Type,
        bid: usize,
        rank: Option<usize>
    }

    impl Hand {
        fn new(cards: String, card_type: Type, bid: usize) -> Self {
            Hand {
                cards,
                card_type,
                bid,
                rank: None,
            }
        }
    }


    //part1
    let mut hands: Vec<Hand> = Vec::new();
    let mut total = 0;
    let card_strength = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

    for line in &lines_vector {
        let line_splitted: Vec<&str> = line.split(" ").collect();

        let mut total = 0usize;
        for card in line_splitted[0].chars() {
            total += line_splitted[0].match_indices(card).count();
        }

        let card_type = match total {
            5 => Type::HighCard,
            7 => Type::OnePair,
            9 => Type::TwoPair,
            11 => Type::ThreeOfAKind,
            13 => Type::FullHouse,
            17 => Type::FourOfAKind,
            25 => Type::FiveOfAKind,
            _ => panic!()
        };

        hands.push(Hand::new(line_splitted[0].to_string(), card_type, line_splitted[1].parse::<usize>().unwrap()));
    }

    for i in 0..hands.len() {
        let mut rank = 1;
        for j in 0..hands.len() {
            if hands[i].cards == hands[j].cards || hands[i].card_type != hands[j].card_type {
                if hands[i].card_type < hands[j].card_type {
                    rank += 1
                }
                continue
            } else {
                for k in 0..5 {
                    if card_strength.iter().position(|x| *x == hands[i].cards.as_bytes()[k] as char) < card_strength.iter().position(|x| *x == hands[j].cards.as_bytes()[k] as char) {
                        rank += 1;
                        break
                    } else if card_strength.iter().position(|x| *x == hands[i].cards.as_bytes()[k] as char) > card_strength.iter().position(|x| *x == hands[j].cards.as_bytes()[k] as char) {
                        break
                    } else {
                        continue
                    }
                }
            }
        }
        hands[i].rank = Some(rank);
        total += hands[i].rank.unwrap() * hands[i].bid
    }

    println!("Total: {:?} (part1)", total);

    //part2

    let card_strength = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

    let mut hands: Vec<Hand> = Vec::new();
    let mut total = 0;

    for line in &lines_vector {
        let line_splitted: Vec<&str> = line.split(" ").collect();

        let mut total = 0usize;
        let j_count = line_splitted[0].chars().filter(|x| x == &'J').count();
        for card in line_splitted[0].chars() {
            total += line_splitted[0].match_indices(card).count();
        }

        let mut card_type = match total {
            5 => Type::HighCard,
            7 => Type::OnePair,
            9 => Type::TwoPair,
            11 => Type::ThreeOfAKind,
            13 => Type::FullHouse,
            17 => Type::FourOfAKind,
            25 => Type::FiveOfAKind,
            _ => panic!()
        };

        if j_count >= 1 {
            card_type = match card_type {
                Type::HighCard => Type::OnePair,
                Type::OnePair => Type::ThreeOfAKind,
                Type::TwoPair => {
                    match j_count {
                        1 => Type::FullHouse,
                        2 => Type::FourOfAKind,
                        _ => panic!()
                    }
                },
                Type::ThreeOfAKind => Type::FourOfAKind,
                Type::FullHouse => Type::FiveOfAKind,
                Type::FourOfAKind => Type::FiveOfAKind,
                Type::FiveOfAKind => Type::FiveOfAKind,
            };
        }

        hands.push(Hand::new(line_splitted[0].to_string(), card_type, line_splitted[1].parse::<usize>().unwrap()));
    }

    for i in 0..hands.len() {
        let mut rank = 1;
        for j in 0..hands.len() {
            if hands[i].cards == hands[j].cards || hands[i].card_type != hands[j].card_type {
                if hands[i].card_type < hands[j].card_type {
                    rank += 1
                }
                continue
            } else {
                for k in 0..5 {
                    if card_strength.iter().position(|x| *x == hands[i].cards.as_bytes()[k] as char) < card_strength.iter().position(|x| *x == hands[j].cards.as_bytes()[k] as char) {
                        rank += 1;
                        break
                    } else if card_strength.iter().position(|x| *x == hands[i].cards.as_bytes()[k] as char) > card_strength.iter().position(|x| *x == hands[j].cards.as_bytes()[k] as char) {
                        break
                    } else {
                        continue
                    }
                }
            }
        }
        hands[i].rank = Some(rank);
        total += hands[i].rank.unwrap() * hands[i].bid
    }
    println!("Total: {:?} (part2)", total);
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}