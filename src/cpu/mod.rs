use std::{eprintln, format, fs, println, vec};
use std::alloc;
use amd::eAMDData;
use rusqlite::{params, Connection, Error};
use sysinfo::{CpuRefreshKind, RefreshKind};
use crate::cpu::amd::AMDData;
use crate::cpu::eCPUDetails::Intel;
use crate::cpu::intel::{eIntelData, IntelData};

pub mod intel;
pub mod amd;

pub enum EnumCPUData {
    Intel(eIntelData),
    AMD(eAMDData),
}

pub trait Database: private::Database {
    fn fetch(keyword: &str, column: crate::cpu::EnumCPUData) -> Result<Option<eCPUDetails>, rusqlite::Error>;
    fn gen_db();
}

pub(crate) mod private {
    use std::path::Path;
    use crate::cpu::eCPUDetails;

    pub trait Database {
        const DATABASE: &'static str = "res/db/cpu.db";
        const CPU_INFO_FOLDER: &'static str;
        fn check_if_db_exists() -> bool {
            if Path::new(Self::DATABASE).exists() {
                log::debug!("Database exists");
                Path::new(Self::DATABASE).exists()
            } else {false}
        }
        fn get_file_names(directory: String) -> Result<Vec<String>, std::io::Error> {
            Ok(Vec::new())
        }
        fn save_to_database(files: Vec<String>) -> Result<(), rusqlite::Error>;
        fn split_csv_line(line: &str) -> Vec<String> {
            let mut fields = Vec::new();
            let mut current_field = String::new();
            let mut in_quotes = false;

            for c in line.chars() {
                if c == '"' {
                    if in_quotes {
                        // End of quoted field
                        in_quotes = false;
                    } else {
                        // Start of quoted field
                        in_quotes = true;
                    }
                } else if c == ',' && !in_quotes {
                    // End of unquoted field
                    fields.push(current_field.clone());
                    current_field.clear();
                } else {
                    // Append character to current field
                    current_field.push(c);
                }
            }

            // Handle the last field
            if !current_field.is_empty() {
                fields.push(current_field.clone());
            }

            fields
        }

        fn parse_clock_speed(speed_str: &str) -> usize {
            // Remove "Up to" and "MHz", then parse
            let cleaned = speed_str
                .replace("Up to ", "")
                .replace(" GHz", "")
                .replace(" MHz", "");

            // Check if it's in GHz or MHz
            if cleaned.contains('.') {
                // Convert GHz to MHz
                (cleaned.parse::<f64>().unwrap_or(0.0) * 1000.0) as usize
            } else {
                cleaned.parse::<usize>().unwrap_or(0)
            }
        }
    }
}

#[derive(Debug)]
pub struct CPUDetails {
    cores: usize,
    vendor: String,
    brand: String,
    model: String,
    frequency: usize,
    details: eCPUDetails,
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum eCPUDetails {
    Intel(IntelData),
    AMD(AMDData),
    Else
}

impl CPUDetails {
    pub fn fetch() -> Self {
        let s = sysinfo::System::new_with_specifics(
            RefreshKind::everything().with_cpu(CpuRefreshKind::everything())
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
        let model = Self::get_intel_model(brand.clone().as_str()).unwrap_or_default();

        let details = if vendor == String::from("GenuineIntel") {
            let _temp = IntelData::fetch(
                Self::get_intel_model(&brand).unwrap_or_default().as_str(),
                EnumCPUData::Intel(eIntelData::Name)
            );

            match _temp {
                Ok(thing) => {
                    thing.unwrap_or(eCPUDetails::Else)
                },
                Err(err) => {
                    eprintln!("An error occurred while fetching CPU details: {}", err);
                    eCPUDetails::Else
                }
            }
        } else if vendor == String::from("AuthenticAMD") {

            let _temp = AMDData::fetch(
                Self::get_intel_model(&brand).unwrap_or_default().as_str(),
                EnumCPUData::AMD(eAMDData::Name)
            );
            
            match _temp {
                Ok(thing) => {
                    thing.unwrap_or(eCPUDetails::Else)
                },
                Err(err) => {
                    eprintln!("An error occurred while fetching CPU details: {}", err);
                    eCPUDetails::Else
                }
            }
        } else {
            eCPUDetails::Else
        };

        Self {
            cores,
            vendor,
            brand,
            model,
            frequency,
            details
        }
    }
    
    fn get_intel_model(brand: &str) -> Option<String> {
        // Remove "(TM)" from the brand string
        let brand = brand.replace("(TM)", "").replace("(tm)", "");
        
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

    fn get_amd_model(brand: &str) -> Option<String> {
        // Split brand into words
        let words: Vec<&str> = brand.split_whitespace().collect();
        
        // Look for word containing 3 consecutive digits
        for word in words {
            let mut digit_count = 0;
            let mut chars = word.chars().peekable();
            
            while let Some(c) = chars.next() {
                if c.is_ascii_digit() {
                    digit_count += 1;
                    // Check next two characters for digits
                    if digit_count == 1 {
                        if let (Some(d1), Some(d2)) = (chars.next(), chars.next()) {
                            if d1.is_ascii_digit() && d2.is_ascii_digit() {
                                return Some(word.to_string());
                            }
                        }
                    }
                } else {
                    digit_count = 0;
                }
            }
        }
        None
    }

}

