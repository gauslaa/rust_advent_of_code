fn import_food() -> Vec<Vec<i32>> {
    let content = include_str!("food.txt");
    let mut food_bank: Vec<Vec<i32>> = Vec::new();
    food_bank.push(Vec::new());
    for line in content.lines() {
        if line.trim().is_empty() {
            food_bank.push(Vec::new());
        } else {
            let number: i32 = line.parse().unwrap();
            food_bank.last_mut().unwrap().push(number);
        }
    }
    food_bank
}

fn find_max(nested_list: Vec<Vec<i32>>) -> (i32, i32) {
    let mut gnome = 0;
    let mut max_macro = 0;
    let mut itter = 0;
    for g in nested_list {
        let sum: i32 = g.iter().sum();
        itter = itter + 1;
        if max_macro < sum {
            max_macro = sum;
            gnome = itter;
        }
    }
    (gnome, max_macro)
}

fn main() {
    let food_bank = import_food();

    let (a, b) = find_max(food_bank);
    println!("The gnome with the highest amount of macores is {a}, with: {b}");
}
