use crate::cpu::amd::AMDData;
use crate::cpu::Database;
use crate::cpu::intel::IntelData;

pub mod cpu;
pub mod os;
pub(crate) mod utils;
pub mod gpu;
pub(crate) mod tests;

pub fn init() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();
    IntelData::gen_db();
    AMDData::gen_db();
}



