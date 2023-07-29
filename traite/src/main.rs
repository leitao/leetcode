use rand::Rng;

#[derive(Debug)]
struct User {
    pub name: String,
    pub age: i32,
}

pub trait HasAge {
    fn in_secs(&self) -> u64;
}

impl HasAge for User {
    fn in_secs(&self) -> u64 {
        return (self.age as u64) * 60 * 60 * 60 * 24 * 360;
    }
}

fn print(obj: impl HasAge) -> Result<i8, i8> {
    println!("Age in secs {0}", obj.in_secs());

    return Ok(10);
}

fn main() {
    let mut rng = rand::thread_rng();

    let u = User {
        name: String::from("Breno"),
        age: 40,
    };
    println!("Name = {0}", u.name);
    let r = print(u);
    let x = r.ok();
    let y = r.unwrap();
    println!("Ret is {:?} {:?} {:?} {}", x, r, x, y);

    println!("Randong is {}", rng.gen_range(0..100));
}
