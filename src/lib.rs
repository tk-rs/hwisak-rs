use std::env;

extern crate cpuid;

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
    pub vendor: String,
    pub brand: String,
    pub generation: u32,
    pub codename: String,
    pub num_cores: u32,
    pub num_logical_cpus: u32,
    pub total_logical_cpus: u32,
}

impl CPU {
    pub fn fetch() -> Self {
        match cpuid::identify() {
            Ok(info) => {
                let generation = Self::extract_intel_processor_generation(&info.brand).unwrap_or_else(|| 0);

                Self {
                    vendor: info.vendor,
                    brand: info.brand,
                    generation,
                    codename: info.codename,
                    num_cores: info.num_cores as u32,
                    num_logical_cpus: info.num_logical_cpus as u32,
                    total_logical_cpus: info.total_logical_cpus as u32,
                }
            }
            Err(err) => {
                Self {
                    ..Self::error()
                }
            }
        }
    }

    fn error() -> Self {
        Self {
            vendor: String::from("Error"),
            brand: String::from("Error"),
            generation: 0,
            codename: String::from("Error"),
            num_cores: 0,
            num_logical_cpus: 0,
            total_logical_cpus: 0,
        }
    }

    fn extract_intel_processor_generation(processor_name: &str) -> Option<u32> {
        // Split the string by whitespace
        let parts: Vec<&str> = processor_name.split_whitespace().collect();

        // Find the part containing i3, i5, i7, or i9
        let processor_identifier = parts
            .iter()
            .find(|&part| part.starts_with("i3") || part.starts_with("i5") ||
                part.starts_with("i7") || part.starts_with("i9"))?;

        // Split the identifier into model family and rest
        let (model_family, _) = processor_identifier.split_at(2);

        // Extract the generation number (first digit)
        let generation = model_family[1..2].parse().ok()?;

        Some(generation)
    }
}


