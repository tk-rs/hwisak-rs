use hwinfo_rs::{cpu, OS};
use hwinfo_rs::cpu::{intel, Database};
use hwinfo_rs::cpu::intel::{eIntelData, sIntelData};

fn main() {
    let os = OS::fetch();
    println!("{:?}", os);

    let cpu = sIntelData::gen_db();
    let details = sIntelData::fetch("i7-1165G7", eIntelData::product);
    println!("{:?}", details);
    
}