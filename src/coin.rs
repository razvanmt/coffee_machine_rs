#[derive(Debug)]
pub struct Treasury {
    amount: i32,
}

#[derive(Debug)]
pub enum Coin {
    Quarter,
    Dime,
    Nickle,
    Penny,
}

impl Coin {
    pub fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Quarter => 25,
            Coin::Dime => 10,
            Coin::Nickle => 5,
            Coin::Penny => 1,
        }
    }
}
