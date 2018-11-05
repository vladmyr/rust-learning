#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // Arizona,
    // Arkansas,
    // California,
    // Colorado,
    // Connecticut,
    // Delaware,
    // Florida,
    // Georgia,
    // Hawaii,
    // Idaho,
    // Illinois,
    // Indiana,
    // Iowa,
    // Kansas,
    // Kentucky,
    // Louisiana,
    // Maine,
    // Maryland,
    // Massachusetts,
    // Michigan,
    // Minnesota,
    // Mississippi,
    // Missouri,
    // Montana,
    // Nebraska,
    // Nevada,
    // NewHampshire,
    // NewJersey,
    // NewMexico,
    // NewYork,
    // NorthCarolina,
    // NorthDakota,
    // Ohio,
    // Oklahoma,
    // Oregon,
    // Pennsylvania,
    // RhodeIsland,
    // SouthCarolina,
    // SouthDakota,
    // Tennessee,
    // Texas,
    // Utah,
    // Vermont,
    // Virginia,
    // Washington,
    // WestVirginia,
    // Wisconsin,
    // Wyoming
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter0 = Coin::Quarter(UsState::Alaska);
    let quarter1 = Coin::Quarter(UsState::Alabama);

    println!("The coin has a value of {} cents", value_in_cents(penny));
    println!("The coin has a value of {} cents", value_in_cents(nickel));
    println!("The coin has a value of {} cents", value_in_cents(dime));
    println!("The coin has a value of {} cents", value_in_cents(quarter0));
    println!("The coin has a value of {} cents", value_in_cents(quarter1));

    // matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Option::Some(value) = six {
        println!("Option::Some of variable six is {}", value);
    }

    if let Option::None = none {
        println!("Option::None of variable none was matched");
    }

    // `_` placeholder
    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),    // `_` will match all the possible cases that weren't 
                    // specified before. `()` is unit value, so nothing will
                    // happen
    }

    // `if let` control flow is less verbose equivalent of `match`
    // caveat: you lose the exhaustive checking that `match` enforces
    let another_u8_value = Some(0u8);
    if let Some(3) = another_u8_value {
        println!("three");
    }

    let coin = Coin::Penny;
    let mut count = 0;
    
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("The count is {}", count);
}
