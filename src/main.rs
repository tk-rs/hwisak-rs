use hwisak_rs::cpu::amd::{eAMDData, AMDData};
use hwisak_rs::cpu::{CPUDetails, Database, EnumCPUData};

fn main() {
    println!("{:?}", CPUDetails::fetch());
    
    println!("{:?}", AMDData::fetch("8700GE", EnumCPUData::AMD(eAMDData::Name)))
}