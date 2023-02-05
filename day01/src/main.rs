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

fn find_max(nested_list: &Vec<Vec<i32>>) -> (usize, i32) {
    let mut gnome = 0;
    let mut max_macro = 0;
    let mut itter: usize = 0;
    for g in nested_list {
        let sum: i32 = g.iter().sum();
        itter += 1;
        if max_macro < sum {
            max_macro = sum;
            gnome = itter;
        }
    }
    (gnome, max_macro)
}

fn top_tree_gnomes(nested_list: &mut Vec<Vec<i32>>) -> i32 {
    let mut total_marco: i32 = 0;
    for _i in [1, 2, 3] {
        let (a, b) = find_max(&nested_list);
        for index in 0..nested_list[a].len() {
            nested_list[a + 1][index] = 0;
            println!("{}", nested_list[a + 1][index]);
        }
        println!("{}", a);
        println!("{}", b);
        total_marco += b;
    }
    total_marco
}

fn main() {
    let mut food_bank = import_food();

    let total_marco = top_tree_gnomes(&food_bank);
    println!("{}", total_marco)
}
