use std::io;
use rand::{rngs::ThreadRng};
use rand::seq::SliceRandom;

fn main() {
    println!("Tick Tac Toe - Rust Edition!");
    let rng = rand::thread_rng();
    let mut set_znakow = [" ", " ", " ", 
                                    " ", " ", " ", 
                                    " ", " ", " "];
    let mut wygrana = false;
    while !wygrana {
        let mut inp = get_input_num();
        if inp == 0 {
            inp = 1;
        }
        if set_znakow[(inp as usize) - 1] == " " {
            set_znakow[(inp as usize) - 1] = "X";
        } else {
            println!("Nie kradnij złodziejaszku ;)");
            continue;
        }
        println!("\"AI\" myśli...");
        for _timeout in 0..1000000 {}
        set_znakow[rand_ask(set_znakow, rng.clone())] = "O";
        plansza(set_znakow);
        let temp = czy_wygrana(set_znakow);
        wygrana = temp != "brak";
        if wygrana {
            while wygrana {
                if temp == "X" {
                    println!("krzyżyk wygrał!!!");
                } else if temp == "O" {
                    println!("kółko wygrało!!! (ale z ciebie bambik)");
                } else {
                    println!("ale z ciebie bambik zremisowałeś")
                }
            }
        }
    }
}

// 1 2 3
// 4 5 6
// 7 8 9

fn czy_wygrana(plns : [&str; 9]) -> &str {
    for i in 0..3 {
        if plns[i * 3 + 0] == plns[i * 3 + 1] && plns[i * 3 + 1] == plns[i * 3 + 2] && plns[i * 3 + 2] == plns[i * 3 + 0] && plns[i * 3 + 1] != " " {
            return plns[i * 3 + 0];
        }
    }
    for i in 0..3 {
        if plns[i + 3 * 0] == plns[i + 3 * 1] && plns[i + 3 * 1] == plns[i + 3 * 2] && plns[i + 3 * 2] == plns[i + 3 * 0]&& plns[i + 3 * 1] != " "  {
            return plns[i + 3 * 0];
        }
    }
    if plns[0] == plns[5] && plns[8] == plns[5] && plns[0] == plns[8] && plns[5] != " "  {
        return plns[5];
    }
    if plns[2] == plns[5] && plns[6] == plns[5] && plns[2] == plns[6] && plns[5] != " " {
        return plns[5];
    }
    let mut tempbul = false;
    for i in 0..9 {
        if plns[i] == " " {
            tempbul = true;
        }
    }
    if !tempbul {
        return "remis";
    }
    return "brak";
}

fn get_input_num() -> i64 {
    let mut input = String::new();
    println!("Number: ");
    io::stdin().read_line(&mut input).expect("Failed to read :(");
    let num: i64 = match input.trim().parse() {
        Ok(parsed_value) => parsed_value,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return get_input_num();
        }
    };

    if num >= 9 {
        println!("Too much O_O");
        return get_input_num();
    }

    return num;
}



fn plansza(plns : [&str; 9]) {
    for i in 1..6 {
        if i == 2 || i == 4 {
            println!("---------")
        } else {
            let mut real_int = 1;
            if i == 1 {
                real_int = 0;
            } else if i == 3 {
                real_int = 1;
            } else if i == 5 {
                real_int = 2;
            }
            println!("{} | {} | {}", plns[real_int * 3], plns[real_int * 3 + 1], plns[real_int * 3 + 2]);
        }
    }
}

fn rand_ask(plns : [&str; 9], mut rng : ThreadRng) -> usize {
    let mut temp_array: Vec<usize> = Vec::new();
    for pole in 0..plns.len() {
        if plns[pole] == " " {
            temp_array.push(pole);
        }
    }
    temp_array.shuffle(&mut rng);
    return *temp_array.get(0).unwrap();
}
