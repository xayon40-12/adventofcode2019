fn main() {
    let file = std::fs::read_to_string("input.txt").expect("Could not load file");
    let file = file.lines().map(|i| i.parse::<i32>().unwrap()/3-2);

    let part1 = file.clone().fold(0, |a,i| a+i);
    println!("{}", part1);

    let part2 = file.fold(0, |a,i| a+mass_fuel(i)+i);
    println!("{}", part2);
}

fn mass_fuel(mass: i32) -> i32 {
    let m = mass/3-2;
    if m>0 {
        mass_fuel(m)+m
    } else {
        0
    }
}
