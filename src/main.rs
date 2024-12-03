use hwinfo_rs::{CPU, OS};

fn main() {
    let os = OS::fetch();
    println!("{:?}", os);

    let cpu = CPU::fetch();
    println!("{:?}", cpu);
}