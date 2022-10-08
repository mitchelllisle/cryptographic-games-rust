use std::io;

fn main() {
    println!("Welcome to the game of love <3 <3 <3");

    println!("Alice, do you want to date Bob? yes/no");
    let alice = match read_input().as_str() {
        "yes\n" => Alice { secret_input: true },
        "no\n" => Alice {
            secret_input: false,
        },
        _ => {
            println!("error reading Alice input");
            return;
        }
    };
    let alice_deck = alice.encode_alice_input();

    println!("Bob, do you want to date Alice? [yes/no]");
    let bob = match read_input().as_str() {
        "yes\n" => Bob { secret_input: true },
        "no\n" => Bob {
            secret_input: false,
        },
        _ => {
            println!("error reading Bob input");
            return;
        }
    };
    let bob_deck = bob.encode_bob_input();

    let mut deck = Deck::join_decks(alice_deck.clone(), bob_deck.clone());

    println!("Bob please shuffle, please pick number of shuffles [1,2,3,4,5]");
    let bob_shift = match read_input().as_str() {
        "1\n" => 1,
        "2\n" => 2,
        "3\n" => 3,
        "4\n" => 4,
        "5\n" => 5,
        _ => {
            println!("error reading Bob input");
            return;
        }
    };
    deck.cyclic_shift(bob_shift);

    println!("Alice please shuffle, please pick number of shuffles [1,2,3,4,5]");
    let alice_shift = match read_input().as_str() {
        "1\n" => 1,
        "2\n" => 2,
        "3\n" => 3,
        "4\n" => 4,
        "5\n" => 5,
        _ => {
            println!("error reading Bob input");
            return;
        }
    };
    deck.cyclic_shift(alice_shift);

    let res = deck.decode();
    let res_str = match res {
        false => "NO MATCH",
        true => "MATCH <3 <3",
    };
    println!(" match result {:?}", res_str);
}

pub fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            //println!("{} bytes read", n);
            //println!("{}", input);
            input
        }
        Err(error) => {
            println!("error: {}", error);
            return "error".to_string();
        }
    }
}
