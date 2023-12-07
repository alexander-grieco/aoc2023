pub mod cards {

    use std::cmp::Ordering;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Card {
        Value(char),
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HandRank {
        HighCard,
        Pair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    const ORDER: &'static str = "J23456789TQKA";

    impl PartialOrd for Card {
        fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
            let (Card::Value(a), Card::Value(b)) = (self, other);
            let a = ORDER.find(*a);
            let b = ORDER.find(*b);
            a.partial_cmp(&b)
        }
    }

    impl Ord for Card {
        fn cmp(&self, other: &Card) -> Ordering {
            let (Card::Value(a), Card::Value(b)) = (self, other);
            let a = ORDER.find(*a);
            let b = ORDER.find(*b);
            a.cmp(&b)
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct Hand {
        pub hand_rank: HandRank,
        pub cards: [Card; 5],
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.hand_rank.cmp(&other.hand_rank) {
                Ordering::Equal => {
                    for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                        match self_card.cmp(&other_card) {
                            Ordering::Equal => continue,
                            non_equal => return Some(non_equal),
                        }
                    }
                    Some(Ordering::Equal)
                }
                non_equal => Some(non_equal),
            }
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
}
