use aoc_toolkit::Day;
pub struct D7 {}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: i32,
}
impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Custom logic for comparing hands based on their values
        let self_value = self.calculate_value();
        let other_value = other.calculate_value();
        if self_value == other_value {
            return self.card_values().cmp(&other.card_values());
        }

        self_value.cmp(&other_value)
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Eq for Hand<'a> {}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.calculate_value() == other.calculate_value()
    }
}
impl<'a> Hand<'a> {
    // Helper method to calculate the value of a Hand based on the specified rules
    fn card_values(&self) -> Vec<i32> {
        let mut values = Vec::new();
        for card in self.cards.chars() {
            values.push(match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                k => k.to_string().parse::<i32>().unwrap(),
            });
        }
        values
    }
    fn calculate_value(&self) -> u32 {
        let values = [
            "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
        ];

        let card_values: Vec<_> = self.cards.split("").collect();
        let mut value_counts = std::collections::HashMap::new();

        for &value in values.iter() {
            let count = card_values.iter().filter(|&&v| v == value).count() as u32;
            value_counts.insert(value, count);
        }

        // Custom logic to calculate the value of the Hand2
        let five_of_a_kind = value_counts.values().any(|&count| count == 5);
        let four_of_a_kind = value_counts.values().any(|&count| count == 4);
        let three_of_a_kind = value_counts.values().any(|&count| count == 3);
        let two_pairs = value_counts.values().filter(|&&count| count == 2).count() == 2;
        let one_pair = value_counts.values().any(|&count| count == 2);
        let full_house = three_of_a_kind && one_pair;

        if five_of_a_kind {
            6
        } else if four_of_a_kind {
            5
        } else if full_house {
            4
        } else if three_of_a_kind {
            3
        } else if two_pairs {
            2
        } else if one_pair {
            1
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct Hand2<'a> {
    cards: &'a str,
    bid: i32,
}
impl<'a> Ord for Hand2<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Custom logic for comparing Hand2s based on their values
        let self_value = self.calculate_value();
        let other_value = other.calculate_value();
        if self_value == other_value {
            return self.card_values().cmp(&other.card_values());
        }

        self_value.cmp(&other_value)
    }
}

impl<'a> PartialOrd for Hand2<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Eq for Hand2<'a> {}

impl<'a> PartialEq for Hand2<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.calculate_value() == other.calculate_value()
    }
}
impl<'a> Hand2<'a> {
    // Helper method to calculate the value of a hand based on the specified rules
    fn card_values(&self) -> Vec<i32> {
        let mut values = Vec::new();
        for card in self.cards.chars() {
            values.push(match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                k => k.to_string().parse::<i32>().unwrap(),
            });
        }
        values
    }
    fn calculate_value(&self) -> u32 {
        let values = ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2"];

        let card_values: Vec<_> = self.cards.split("").collect();
        let mut value_counts = std::collections::HashMap::new();

        for &value in values.iter() {
            let count = card_values.iter().filter(|&&v| v == value).count() as u32;
            value_counts.insert(value, count);
        }
        let amount_of_jokers: u32 = card_values
            .iter()
            .filter(|x| **x == "J")
            .collect::<Vec<_>>()
            .len() as u32;

        // Custom logic to calculate the value of the hand
        let five_of_a_kind = value_counts
            .values()
            .any(|&count| count == 5 - amount_of_jokers);
        if five_of_a_kind {
            return 6;
        }
        let four_of_a_kind = value_counts
            .values()
            .any(|&count| count == 4 - amount_of_jokers);
        if four_of_a_kind {
            return 5;
        }
        let one_pair = value_counts
            .values()
            .any(|&count| count == 2 - amount_of_jokers);
        let three_of_a_kind = value_counts
            .values()
            .any(|&count| count == 3 - amount_of_jokers);

        let one_pair_no_joker = value_counts.values().any(|&count| count == 2);
        let two_pairs_no_joker = value_counts.values().filter(|&&count| count == 2).count() == 2;
        let two_pairs = (one_pair_no_joker && amount_of_jokers == 1) || two_pairs_no_joker;

        let full_house =
            (two_pairs_no_joker && amount_of_jokers == 1) || (three_of_a_kind && one_pair_no_joker);

        if full_house {
            return 4;
        }
        if three_of_a_kind {
            return 3;
        }
        if two_pairs {
            return 2;
        }
        if one_pair {
            return 1;
        }
        0
    }
}

impl Day for D7 {
    fn part1(&self, data: &str) -> String {
        let lines = data.lines();
        let mut hands = Vec::new();
        for line in lines {
            let vals: Vec<_> = line.split(" ").collect();
            let cards = vals.first().unwrap();
            let bid = vals.last().unwrap().parse::<i32>().unwrap();
            hands.push(Hand { bid, cards })
        }
        hands.sort();
        let mut total = 0;
        for (idx, hand) in hands.iter().enumerate() {
            total += hand.bid * (idx as i32 + 1);
        }
        total.to_string()
    }

    fn part2(&self, data: &str) -> String {
        let lines = data.lines();
        let mut hands = Vec::new();
        for line in lines {
            let vals: Vec<_> = line.split(" ").collect();
            let cards = vals.first().unwrap();
            let bid = vals.last().unwrap().parse::<i32>().unwrap();
            hands.push(Hand2 { bid, cards })
        }
        hands.sort();
        let mut total = 0;
        for (idx, hand) in hands.iter().enumerate() {
            total += hand.bid * (idx as i32 + 1);
        }
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let d = D7 {};
        assert_eq!(
            "6440",
            d.part1(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
            )
        );
    }
    #[test]
    fn test_part2() {
        let d = D7 {};
        assert_eq!(
            "5905",
            d.part2(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
            )
        );
    }
}
