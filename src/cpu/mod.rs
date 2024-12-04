use std::{eprintln, format, fs, println, vec};
use rusqlite::{params, Connection};
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use crate::cpu::intel::eIntelData;
use crate::cpu::intel::eIntelData::product;

pub mod intel;
pub mod amd;

pub trait Database {
    const DATABASE: &'static str;
    fn gen_db();
    fn fetch(keyword: &str, column: crate::cpu::intel::eIntelData) -> Result<Option<crate::cpu::intel::sIntelData>, rusqlite::Error>;
    fn get_file_names(directory: String) -> Result<Vec<String>, std::io::Error>;
    fn save_to_database(files: Vec<String>) -> Result<(), rusqlite::Error>;
    fn split_csv_line(line: &str) -> Vec<String>;
}

#[derive(Debug)]
pub struct CPUDetails {
    cores: usize,
    vendor: String,
    brand: String,
    model: String,
    frequency: usize,
    details: String,
}

impl CPUDetails {
    pub fn fetch() -> Self {
        let s = System::new_with_specifics(
            RefreshKind::new().with_cpu(CpuRefreshKind::everything())
        );

        let mut num_cores = Vec::new();

        let mut vendor = String::new();

        let mut brand = String::new();

        let mut frequency: usize = 0;

        for cpu in s.cpus() {
            vendor = cpu.vendor_id().to_string();
            brand = cpu.brand().to_string();
            frequency = cpu.frequency() as usize;
            num_cores.push(cpu.name())
        }

        let cores = num_cores.len();
        
        let details = String::new();

        let model = Self::get_model(brand.clone().as_str()).unwrap_or_else(|| "".to_string());

        if vendor.contains("Intel") {
            let details = intel::sIntelData::fetch(Self::get_model(&brand).unwrap_or_else(|| "".to_string()).as_str(), product);
        }

        Self {
            cores,
            vendor,
            brand,
            model,
            frequency,
            details
        }
    }
    
    fn get_model(brand: &str) -> Option<String> {
        // Common Intel product lines
        let product_lines = ["Atom", "Core", "Xeon", "Pentium", "Celeron"];
        
        // Try to find product line in brand string
        for line in product_lines {
            if let Some(pos) = brand.find(line) {
                // Get everything after the product line
                let after_line = &brand[pos..];
                
                // Split by spaces and get components
                let parts: Vec<&str> = after_line.split_whitespace().collect();
                
                if parts.len() >= 2 {
                    // Handle different format cases
                    if parts[1].starts_with("i") {
                        // Case: Core i3/i5/i7/i9-XXXXX
                        return Some(format!("{} {}", parts[0], parts[1]));
                    } else if parts[1].contains("-") {
                        // Case: Atom/Pentium/Celeron XXXXX
                        return Some(format!("{} {}", parts[0], parts[1]));
                    } else {
                        // Case: Core 2 XXXXX or other formats
                        let model = parts[..3].join(" ");
                        return Some(model);
                    }
                }
            }
        }
        None
    }

}

