use std::io;

fn get_line() -> Option<String>
{
    let stdin = io::stdin();

    for line in stdin.lines(){
        return Some(line.unwrap());
    }

    return None;
}

fn get_vec<'a>(line: &'a String) -> Vec<i32>{
    let mut vec = Vec::new();

    let first = line.split("[").nth(1).unwrap();
    let second = first.split("]").nth(0).unwrap();

    for element in second.split(",") {
        vec.push(element.trim().parse::<i32>().unwrap());
    }
    println!("{:?}", vec);

    return vec;
}

fn get_target<'a>(line: &'a String) -> &'a str{
    let first = line.split("target =").nth(1).unwrap();
    return first;
}


fn process(vector: &mut Vec<i32>, target: u32){
    vector.sort();
    let size = vector.len() - 1;
    for x in 0..size {
        for  y in x..size {
            let sum = (vector[x] + vector[y]) as u32;
            if sum == target {
                println!("{} {}", x, y);
            }
        }
    }

}

fn main() {
    let line = get_line().unwrap();
    let mut vec = get_vec(&line);
    let target = get_target(&line).trim();

    let x = target.parse::<u32>();
    process(&mut vec, target.parse::<u32>().unwrap());
}
