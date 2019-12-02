fn main() {
    let file = std::fs::read_to_string("input.txt").expect("Could not load file");
    let file = file.trim();
    let prog = file.split(",").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let mut part1 = prog.clone();
    part1[1] = 12;
    part1[2] = 2;
    run(&mut part1);
    println!("{}", part1[0]);
}

fn run(prog: &mut Vec<i32>) {
    let mut i = 0;
    loop {
        let (o,a,b,pos) = (prog[i],prog[prog[i+1]as usize],prog[prog[i+2]as usize],prog[i+3]);
        prog[pos as usize] = match o {
            1 => a+b,
            2 => a*b,
            99 => break,
            _ => panic!("wrong opcode")
        };
        i += 4;
    }
}
