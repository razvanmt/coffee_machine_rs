#[derive(Debug)]
pub struct Coffee {
    name: CoffeeName,
    ingredients: Ingredients,
}

#[derive(Debug)]
pub enum CoffeeName {
    Espresso,
    Latte,
    Cappuccino
}

#[derive(Debug)]
struct Resources {
    name: String,
    quantity: i32,
}

#[derive(Debug)]
struct Ingredients(i32, i32, i32);

#[derive(Debug)]
pub struct InputError;

impl Coffee {
    pub fn new_coffee(name: CoffeeName) -> Option<Coffee> {
        match name {
            CoffeeName::Espresso => Some(
                Coffee {
                name: CoffeeName::Espresso,
                ingredients: Ingredients(150, 0, 18),
            }),
            CoffeeName::Latte => Some(
                Coffee {
                name: CoffeeName::Latte,
                ingredients: Ingredients(200, 150, 24),
            }),
            CoffeeName::Cappuccino => Some(
                Coffee {
                name: CoffeeName::Cappuccino,
                ingredients: Ingredients(250, 100, 24),
            })
        }
    }

    pub fn from_str(s: &str) -> Result<CoffeeName, InputError> {
        match s {
            "Espresso" => Ok(CoffeeName::Espresso),
            "Latte" => Ok(CoffeeName::Latte),
            "Cappuccino" => Ok(CoffeeName::Cappuccino),
            _ => Err(InputError)
        }
    }
}
