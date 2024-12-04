use csv;
use std::fs;
use std::fs::File;
use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct sIntelData {
    product: String,
    status: ProductStatus,
    release_date: String,
    code_name: String,
    cores: Option<usize>,
    threads: Option<usize>,
    lithography: Option<usize>,
    max_turbo_freq: Option<usize>,
    base_freq: Option<usize>,
    thermal_design_power: Option<usize>,
    cache: Option<usize>,
    cache_info: String,
    max_memory_size: Option<usize>,
    memory_types: Vec<String>,
    max_memory_speed: Option<usize>,
    graphics: Option<String>,
}

pub enum eIntelData {
    product,
    status,
    release_date,
    code_name,
    cores,
    threads,
    lithography,
    max_turbo_freq,
    base_freq,
    thermal_design_power,
    cache,
    cache_info,
    max_memory_size,
    memory_types,
    max_memory_speed,
    graphics,
}

impl eIntelData {
    fn to_string(&self) -> &str {
        match self {
            eIntelData::product => "product",
            eIntelData::status => "status",
            eIntelData::release_date => "release_date",
            eIntelData::code_name => "code_name",
            eIntelData::cores => "cores",
            eIntelData::threads => "threads",
            eIntelData::lithography => "lithography",
            eIntelData::max_turbo_freq => "max_turbo_freq",
            eIntelData::base_freq => "base_freq",
            eIntelData::thermal_design_power => "thermal_design_power",
            eIntelData::cache => "cache",
            eIntelData::cache_info => "cache_info",
            eIntelData::max_memory_size => "max_memory_size",
            eIntelData::memory_types => "memory_types",
            eIntelData::max_memory_speed => "max_memory_speed",
            eIntelData::graphics => "graphics",
        }
    }
}

#[derive(Debug, PartialEq)]
enum ProductStatus {
    Launched,
    Discontinued,
    Announced,
}

impl crate::cpu::Database for sIntelData {
    const DATABASE: &'static str = "res/db/cpu.db";

    fn gen_db() {
        let dir = "./res/cpu/intel/intel-processors";
        let folder1 = dir.to_string();
        
        let folder2 = format!("{}/v1_1", folder1);
        
        let folder3 = format!("{}/v1_2", folder1);
        

        let mut files: Vec<String> = Vec::new();
        
       
        let file_list1 = Self::get_file_names(folder1)
            .expect("Error occurred while reading directory")
            .into_iter()
            .filter(|file| file.ends_with(".csv"))
            .collect::<Vec<_>>();
        let file_list2 = Self::get_file_names(folder2)
            .expect("Error occurred while reading directory")
            .into_iter()
            .filter(|file| file.ends_with(".csv"))
            .collect::<Vec<_>>();
        let file_list3 = Self::get_file_names(folder3)
            .expect("Error occurred while reading directory")
            .into_iter()
            .filter(|file| file.ends_with(".csv"))
            .collect::<Vec<_>>();
        
        let mut temp: Vec<String> = Vec::new();
        for file in file_list1 {
            let thing = format!("./res/cpu/intel/intel-processors/{}", file);
            temp.push(thing);
        }

        for file in file_list2 {
            let thing = format!("./res/cpu/intel/intel-processors/v1_1/{}", file);
            temp.push(thing);
        }

        for file in file_list3 {
            let thing = format!("./res/cpu/intel/intel-processors/v1_2/{}", file);
            temp.push(thing);
        }
        
        
        // for folder in folders {
        //     let file_list = Self::get_file_names(folder)
        //         .expect("Error occurred while reading directory")
        //         .into_iter()
        //         .filter(|file| file.ends_with(".csv"))
        //         .collect::<Vec<_>>();
        // 
        //     for file in file_list {
        //         files.push();
        //     }
        // }

        println!("{:#?}", temp);
        
        Self::save_to_database(temp).expect("Failed to save to database");
    }
    
    fn fetch(keyword: &str, column: eIntelData) -> Result<Option<sIntelData>, rusqlite::Error> {
        let conn = Connection::open(Self::DATABASE)?;
        
        let query = format!(
            "SELECT * FROM intel_cpus WHERE {} LIKE ?1",
            column.to_string()
        );
    
        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query(params![format!("%{}%", keyword)])?;
    
        if let Some(row) = rows.next()? {
            let intel_data = sIntelData {
                product: row.get(0)?,
                status: match row.get::<_, String>(1)?.as_str() {
                    "Launched" => ProductStatus::Launched,
                    "Discontinued" => ProductStatus::Discontinued,
                    _ => ProductStatus::Announced,
                },
                release_date: row.get(2)?,
                code_name: row.get(3)?,
                cores: row.get::<_, String>(4)?.parse().ok(),
                threads: row.get::<_, String>(5)?.parse().ok(),
                lithography: row.get::<_, String>(6)?.parse().ok(),
                max_turbo_freq: row.get::<_, String>(7)?.parse().ok(),
                base_freq: row.get::<_, String>(8)?.parse().ok(),
                thermal_design_power: row.get::<_, String>(9)?.parse().ok(),
                cache: row.get::<_, String>(10)?.parse().ok(),
                cache_info: row.get(11)?,
                max_memory_size: row.get::<_, String>(12)?.parse().ok(),
                memory_types: row.get::<_, String>(13)?.split(',').map(|s| s.trim().to_string()).collect(),
                max_memory_speed: row.get::<_, String>(14)?.parse().ok(),
                graphics: Some(row.get(15)?),
            };
            Ok(Some(intel_data))
        } else {
            Ok(None)
        }
    }

    fn get_file_names(directory: String) -> Result<Vec<String>, std::io::Error> {
        // Read the directory contents
        let paths = fs::read_dir(directory)?;

        // Collect and convert file names to a vector of Strings
        let file_names: Vec<String> = paths
            .filter_map(|entry| {
                entry.ok().and_then(|dir_entry| {
                    dir_entry.file_name().into_string().ok()
                })
            })
            .collect();

        Ok(file_names)
    }

    fn save_to_database(files: Vec<String>) -> Result<(), rusqlite::Error> {
        // Initialize database connection
        let mut conn = Connection::open(Self::DATABASE)?;

        // Create table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS intel_cpus (
                product TEXT PRIMARY KEY,
                status TEXT,
                release_date TEXT,
                code_name TEXT,
                cores TEXT,
                threads TEXT,
                lithography TEXT,
                max_turbo_freq TEXT,
                base_freq TEXT,
                thermal_design_power TEXT,
                cache TEXT,
                cache_info TEXT,
                max_memory_size TEXT,
                memory_types TEXT,
                max_memory_speed TEXT,
                graphics TEXT
            )",
            [],
        )?;

        let mut intel_items = Vec::new();

        // Existing file parsing logic
        for file in files {
            let file_content = match fs::read_to_string(&file) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file {}: {}", file, e);
                    continue;
                }
            };

            for line in file_content.lines().skip(1) {
                let fields = Self::split_csv_line(line);
                
                // Pad fields array if incomplete
                let mut fields_padded = vec!["N/A".to_string(); 16];
                for (i, field) in fields.iter().enumerate() {
                    if i < 16 {
                        fields_padded[i] = field.to_string();
                    }
                }
            
                let intel_item = sIntelData {
                    product: fields_padded[0].trim().to_string(),
                    status: match fields_padded[1].trim() {
                        "Launched" => ProductStatus::Launched,
                        "Discontinued" => ProductStatus::Discontinued,
                        _ => ProductStatus::Announced,
                    },
                    release_date: fields_padded[2].trim().to_string(),
                    code_name: fields_padded[3].trim().to_string(),
                    cores: fields_padded[4].trim().parse::<usize>().ok(),
                    threads: fields_padded[5].trim().parse::<usize>().ok(),
                    lithography: fields_padded[6].trim().parse::<usize>().ok(),
                    max_turbo_freq: fields_padded[7].trim().parse::<usize>().ok(),
                    base_freq: fields_padded[8].trim().parse::<usize>().ok(),
                    thermal_design_power: fields_padded[9].trim().parse::<usize>().ok(),
                    cache: fields_padded[10].trim().parse::<usize>().ok(),
                    cache_info: fields_padded[11].trim().to_string(),
                    max_memory_size: fields_padded[12].trim().parse::<usize>().ok(),
                    memory_types: if fields_padded[13].trim() == "N/A" { 
                        Vec::new() 
                    } else { 
                        fields_padded[13].split(',').map(|s| s.trim().to_string()).collect() 
                    },
                    max_memory_speed: fields_padded[14].trim().parse::<usize>().ok(),
                    graphics: if fields_padded[15].trim() == "N/A" { 
                        None 
                    } else { 
                        Some(fields_padded[15].trim().to_string()) 
                    },
                };
            
                intel_items.push(intel_item);
            }
        }

        // Database transaction
        let tx = conn.transaction()?;
        {
            {
                let mut stmt = tx.prepare(
                    "INSERT OR REPLACE INTO intel_cpus (
                        product, status, release_date, code_name, cores, threads,
                        lithography, max_turbo_freq, base_freq, thermal_design_power,
                        cache, cache_info, max_memory_size, memory_types,
                        max_memory_speed, graphics
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)"
                )?;
            
                for item in &intel_items {  // Use reference to avoid moving item
                    stmt.execute(params![
                        &item.product,
                        &format!("{:?}", item.status),
                        &item.release_date,
                        &item.code_name,
                        &item.cores.map(|v| v.to_string()).unwrap_or_default(),
                        &item.threads.map(|v| v.to_string()).unwrap_or_default(),
                        &item.lithography.map(|v| v.to_string()).unwrap_or_default(),
                        &item.max_turbo_freq.map(|v| v.to_string()).unwrap_or_default(),
                        &item.base_freq.map(|v| v.to_string()).unwrap_or_default(),
                        &item.thermal_design_power.map(|v| v.to_string()).unwrap_or_default(),
                        &item.cache.map(|v| v.to_string()).unwrap_or_default(),
                        &item.cache_info,
                        &item.max_memory_size.map(|v| v.to_string()).unwrap_or_default(),
                        &item.memory_types.join(","),
                        &item.max_memory_speed.map(|v| v.to_string()).unwrap_or_default(),
                        &item.graphics.as_deref().unwrap_or_default()
                    ])?;
                }
            }
            tx.commit()?;
            
            println!("Successfully saved {} items to database", intel_items.len());
        }
        Ok(())
    }

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
}