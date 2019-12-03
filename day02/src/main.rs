fn main() {
    let file = std::fs::read_to_string("input.txt").expect("Could not load file");
    let file = file.trim();
    let prog = file.split(",").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<_>>();

    println!("part1: {}", run(&prog, 12, 2));

    for noun in 0..99 {
        for verb in 0..99 {
            if run(&prog, noun, verb) == 19690720 {
                println!("part2: {}", 100*noun + verb);
            }
        }
    }
}

fn run(prog: &Vec<i32>, noun: i32, verb: i32) -> i32{
    let mut i = 0;
    let mut p = prog.clone();
    p[1] = noun;
    p[2] = verb;
    loop {
        let (o,a,b,pos) = (p[i],p[p[i+1]as usize],p[p[i+2]as usize],p[i+3]);
        p[pos as usize] = match o {
            1 => a+b,
            2 => a*b,
            99 => break,
            _ => panic!("wrong opcode")
        };
        i += 4;
    }
    p[0]
}
