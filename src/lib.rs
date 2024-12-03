use std::any::Any;
use std::env;
use raw_cpuid::{CpuId, FeatureInfo};
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use regex::Regex;

#[derive(Debug)]
pub struct OS {
    os_type: String,
    version: String,
    edition: String,
    codename: String,
    bitness: String,
    architecture: String,
}

impl OS {
    pub fn fetch() -> Self {
        let info = os_info::get();

        let os_type = info.os_type().to_string();
        let version = info.version().to_string();
        let edition: String = info.edition().unwrap_or("Unavailable").to_string();
        let codename = info.codename().unwrap_or("Unavailable").to_string();
        let bitness = info.bitness().to_string();
        let architecture = info.architecture().unwrap_or("Unavailable").to_string();

        Self {
            os_type,
            version,
            edition,
            codename,
            bitness,
            architecture,
        }
    }
}

#[derive(Debug)]
pub struct CPU {
    cores: usize,
    vendor: String,
    brand: String,
    frequency: usize,
    details: Option<eCPUDetails>,
}

#[derive(Debug)]
enum eCPUDetails {
    Intel(sCPUDetails),
    Amd(sCPUDetails),
}

trait Intel {
    fn extract(processor_name: String) -> Option<sCPUDetails>;
}

trait AMD {
    fn extract(processor_name: String) -> Option<AmdCPU>;
}

#[derive(Debug)]
struct sCPUDetails {
    performance_tier: String,
    generation: u8,
    model: String,
    suffix: String,
}

impl CPU {
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
        
        let mut details;
        
        if brand.contains("Ryzen") {
            details = AmdCPU::extract(brand);
        } else if brand.contains("Intel") {
            details = IntelCPU::extract(brand);
        }
        
        match details {
            Some(value) => {details = value},
            None => {}
        }
        
        Self {
            cores,
            vendor,
            brand,
            frequency,
            details
        }
    }
}

impl Intel for sCPUDetails {
    fn extract(processor_name: String) -> Option<sCPUDetails> {
        // Split the string by whitespace
        let parts: Vec<&str> = processor_name.split_whitespace().collect();

        // Find the part containing i3, i5, i7, or i9
        let processor_identifier = parts
            .iter()
            .find(|&part| part.starts_with("i3") || part.starts_with("i5") ||
                part.starts_with("i7") || part.starts_with("i9"))?;

        // Split the processor identifier into performance tier and model details
        let performance_tier = processor_identifier[..2].to_string();

        // Find the full model number part (e.g., 14900K)
        let model_number_part = parts
            .iter()
            .find(|&part| part.chars().any(|c| c.is_digit(10)))?;

        // Split the model number into components
        let mut chars = model_number_part.chars();

        // Extract generation (first digit)
        let generation = chars.next()?.to_digit(10)?;

        // Collect model and suffix
        let remaining: String = chars.collect();

        // Split remaining into model and suffix
        let (model, suffix) = if let Some(suffix_index) = remaining.find(|c: char| !c.is_digit(10)) {
            (&remaining[..suffix_index], &remaining[suffix_index..])
        } else {
            (&remaining[..], "")
        };

        Some(eCPUDetails::Intel(sCPUDetails {
            performance_tier,
            generation: generation as u8,
            model: model.to_string(),
            suffix: suffix.to_string(),
            }))
    }
}

impl AMD for sCPUDetails {
    fn extract(processor_name: String) -> Option<sCPUDetails> {
        let parts: Vec<&str> = processor_name.split_whitespace().collect();

        // Find the Ryzen performance tier
        let performance_tier_part = parts
            .iter()
            .find(|&part| part == &"3" || part == &"5" || part == &"7" || part == &"9")?;
        let performance_tier = performance_tier_part.parse::<String>().ok()?;

        // Find the model number part (containing generation and model)
        let model_number_part = parts
            .iter()
            .find(|&part| part.chars().all(|c| c.is_digit(10) || c.is_alphabetic()))?;

        let mut chars = model_number_part.chars();

        let generation = chars.next()?.to_digit(10)?;

        let remaining: String = chars.collect();

        let (model, suffix) = if let Some(suffix_index) = remaining.find(|c: char| !c.is_digit(10)) {
            (&remaining[..suffix_index], &remaining[suffix_index..])
        } else {
            (&remaining[..], "")
        };

        Some(sCPUDetails {
            performance_tier,
            generation: generation as u8,
            model: model.to_string(),
            suffix: suffix.to_string(),
        })
    }
}




