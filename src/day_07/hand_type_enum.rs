#[derive(Clone)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum HandType {
    FiveOfAKind = 1,
    FourOfAKind = 2,
    FullHouse = 3,
    ThreeOfAKind = 4,
    TwoOfAKind = 5,
    OneOfAKind = 6,
    HighCard = 7
}