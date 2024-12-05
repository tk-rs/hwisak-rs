use env_logger::init;
use hwisak_rs::cpu::amd::{eAMDData, AMDData};
use hwisak_rs::cpu::{CPUDetails, Database, EnumCPUData};
use hwisak_rs::gpu::GPUDetails;
use hwisak_rs::os::OSDetails;

fn main() {
    hwisak_rs::init();
    println!(r"

    Machine info using hwisak-rs library
    Made by tk-rs
   =============
   CPU Details: {:#?}
   -------------
   OS Details: {:#?}
   -------------
   GPU Details: {:#?}
   -------------
    ",
             CPUDetails::fetch(),
             OSDetails::fetch(),
             GPUDetails::fetch(),
    );
}