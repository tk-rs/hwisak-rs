use std::fs;
use rusqlite::{params, Connection, Error};
use crate::cpu::{eCPUDetails, EnumCPUData};
use crate::cpu::intel::eIntelData;
use crate::cpu::private::Database;

#[derive(Debug)]
pub struct AMDData {
    pub name: String,
    pub family: String,
    pub series: String,
    pub formFactor: FormFactor,
    pub cores: usize,
    pub threads: usize,
    pub boost_clock: usize,
    pub base_clock: usize,
    pub L1Cache: String,
    pub L2Cache: String,
    pub L3Cache: String,
    pub DefaultTDP: String,
    pub AMDConfigurableTDP: String,
    pub lithography: String,
    pub overclocking_enabled: Option<bool>,
    pub cpu_socket: String,
    pub PIB: String,
    pub MPK: String,
    pub recommended_cooler: Option<String>,
    pub operating_temperature_max: usize,
    pub launch_date: String,
    pub os_support: Vec<String>,
    pub PCI_Express_version: String,
    pub system_memory_type: String,
    pub memory_channels: usize,
    pub system_memory_specification: String,
    pub graphics: Graphics,
    pub AMD_RyzenAIEnabled: Option<bool>,
    pub product_id: ProductID,
    pub supported_technologies: Vec<String>
}

#[derive(Debug)]
struct ProductID {
    pub boxed: Option<String>,
    pub tray: Option<String>,
    pub mpk: Option<String>,
}

#[derive(Debug)]
pub struct Graphics {
    pub model: String,
    pub cores: usize,
    pub frequency: usize,
}

#[derive(Debug)]
pub enum FormFactor {
    Laptops,
    Desktops,
    BoxedProcessor,
    // 1L Desktops
    TinyDesktops,
    MobileWorkstations,
    Handheld,
}

pub enum eAMDData {
    Name,
    Family,
    Series,
    FormFactor,
    Cores,
    Threads,
    MaxBoostClock,
    BaseClock,
    L2Cache,
    L3Cache,
    TDP,
    L1Cache,
    ConfigurableTDP,
    ProcessorTechnology,
    Socket,
    LaunchDate,
    GraphicsModel,
}

impl eAMDData {
    fn to_string(&self) -> &str {
        match self {
            eAMDData::Name => "name",
            eAMDData::Family => "family",
            eAMDData::Series => "series",
            eAMDData::FormFactor => "form_factor",
            eAMDData::Cores => "cores",
            eAMDData::Threads => "threads",
            eAMDData::MaxBoostClock => "max_boost_clock",
            eAMDData::BaseClock => "base_clock",
            eAMDData::L2Cache => "l2_cache",
            eAMDData::L3Cache => "l3_cache",
            eAMDData::TDP => "tdp",
            eAMDData::L1Cache => "l1_cache",
            eAMDData::ConfigurableTDP => "configurable_tdp",
            eAMDData::ProcessorTechnology => "processor_technology",
            eAMDData::Socket => "socket",
            eAMDData::LaunchDate => "launch_date",
            eAMDData::GraphicsModel => "graphics_model",
        }
    }
}

impl AMDData {
}

impl crate::cpu::Database for AMDData {
    fn fetch(keyword: &str, column: EnumCPUData) -> Result<Option<eCPUDetails>, rusqlite::Error> {
        Self::gen_db();

        let column = match column {
            EnumCPUData::Intel(_) => panic!("Cannot use an Intel enum for an AMD query"),
            EnumCPUData::AMD(amd_col) => amd_col
        };
        let conn = Connection::open(Self::DATABASE)?;

        let query = format!(
            "SELECT * FROM amd_cpus WHERE {} LIKE ?1",
            column.to_string()
        );


        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query(params![format!("%{}%", keyword)])?;


        if let Some(row) = rows.next()? {
            let amd_data = AMDData {
                name: row.get(0).unwrap_or_else(|e| {
                    println!("Error getting name: {}", e);
                    "Unknown".to_string()
                }),
                family: row.get(1).unwrap_or_else(|e| {
                    println!("Error getting family: {}", e);
                    "Unknown".to_string()
                }),
                series: row.get(2).unwrap_or_else(|e| {
                    println!("Error getting series: {}", e);
                    "Unknown".to_string()
                }),
                formFactor: match row.get::<_, String>(3).unwrap_or_else(|e| {
                    println!("Error getting formFactor: {}", e);
                    "Unknown".to_string()
                }).as_str() {
                    "Laptops" => FormFactor::Laptops,
                    "Desktops" => FormFactor::Desktops,
                    "BoxedProcessor" => FormFactor::BoxedProcessor,
                    "TinyDesktops" => FormFactor::TinyDesktops,
                    "MobileWorkstations" => FormFactor::MobileWorkstations,
                    "Handheld" => FormFactor::Handheld,
                    _ => {
                        println!("Unknown FormFactor: {}", row.get::<_, String>(3).unwrap_or_else(|e| e.to_string()));
                        FormFactor::Desktops // Default to Desktops if unknown
                    }
                },
                cores: row.get::<_, String>(4).unwrap_or_else(|e| {
                    println!("Error getting cores: {}", e);
                    "0".to_string()
                }).parse().unwrap_or(0), // Parse to usize, handle errors.
                threads: row.get::<_, String>(5).unwrap_or_else(|e| {
                    println!("Error getting threads: {}", e);
                    "0".to_string()
                }).parse().unwrap_or(0),
                boost_clock: row.get::<_, String>(6).unwrap_or_else(|e| {
                    println!("Error getting boost_clock: {}", e);
                    "0".to_string()
                }).parse().unwrap_or(0),
                base_clock: row.get::<_, String>(7).unwrap_or_else(|e| {
                    println!("Error getting base_clock: {}", e);
                    "0".to_string()
                }).parse().unwrap_or(0),
                L1Cache: row.get(8).unwrap_or_else(|e| {
                    println!("Error getting L1Cache: {}", e);
                    "Unknown".to_string()
                }),
                L2Cache: row.get(9).unwrap_or_else(|e| {
                    println!("Error getting L2Cache: {}", e);
                    "Unknown".to_string()
                }),
                L3Cache: row.get(10).unwrap_or_else(|e| {
                    println!("Error getting L3Cache: {}", e);
                    "Unknown".to_string()
                }),
                DefaultTDP: row.get(11).unwrap_or_else(|e| {
                    println!("Error getting DefaultTDP: {}", e);
                    "Unknown".to_string()
                }),
                AMDConfigurableTDP: row.get(12).unwrap_or_else(|e| {
                    println!("Error getting AMDConfigurableTDP: {}", e);
                    "Unknown".to_string()
                }),
                lithography: row.get(13).unwrap_or_else(|e| {
                    println!("Error getting lithography: {}", e);
                    "Unknown".to_string()
                }),
                overclocking_enabled: row.get(14).ok(),
                cpu_socket: row.get(15).unwrap_or_else(|e| {
                    println!("Error getting cpu_socket: {}", e);
                    "Unknown".to_string()
                }),
                PIB: row.get(16).unwrap_or_else(|e| {
                    println!("Error getting PIB: {}", e);
                    "Unknown".to_string()
                }),
                MPK: row.get(17).unwrap_or_else(|e| {
                    println!("Error getting MPK: {}", e);
                    "Unknown".to_string()
                }),
                recommended_cooler: row.get(18).ok(),
                operating_temperature_max: row.get::<_, String>(19).unwrap_or_else(|e| {
                    println!("Error getting operating_temperature_max: {}", e);
                    "0".to_string()
                }).parse().unwrap_or(0),
                launch_date: row.get(20).unwrap_or_else(|e| {
                    println!("Error getting launch_date: {}", e);
                    "Unknown".to_string()
                }),
                os_support: row.get(21).unwrap_or_else(|e| {
                    println!("Error getting os_support: {}", e);
                    "".to_string()
                }).split(',').map(|s| s.trim().to_string()).collect(),
                PCI_Express_version: row.get(22).unwrap_or_else(|e| {
                    println!("Error getting PCI_Express_version: {}", e);
                    "Unknown".to_string()
                }),
                system_memory_type: row.get(23).unwrap_or_else(|e| {
                    println!("Error getting system_memory_type: {}", e);
                    "Unknown".to_string()
                }),
                memory_channels: row.get::<_, String>(24).unwrap_or_else(|e| {
                    println!("Error getting memory_channels: {}", e);
                    "0".to_string()
                }).parse().unwrap_or(0),
                system_memory_specification: row.get(25).unwrap_or_else(|e| {
                    println!("Error getting system_memory_specification: {}", e);
                    "Unknown".to_string()
                }),
                graphics: Graphics {
                    model: row.get(26).unwrap_or_else(|e| {
                        println!("Error getting graphics model: {}", e);
                        "Unknown".to_string()
                    }),
                    cores: row.get::<_, String>(27).unwrap_or_else(|e| {
                        println!("Error getting graphics cores: {}", e);
                        "0".to_string()
                    }).parse().unwrap_or(0),
                    frequency: row.get::<_, String>(28).unwrap_or_else(|e| {
                        println!("Error getting graphics frequency: {}", e);
                        "0".to_string()
                    }).parse().unwrap_or(0),
                },
                AMD_RyzenAIEnabled: row.get(29).ok(),
                product_id: ProductID {
                    boxed: row.get(30).ok(),
                    tray: row.get(31).ok(),
                    mpk: row.get(32).ok(),
                },
                supported_technologies: row.get(33).unwrap_or_else(|e| {
                    println!("Error getting supported_technologies: {}", e);
                    "".to_string()
                }).split(',').map(|s| s.trim().to_string()).collect(),
            };

            Ok(Some(eCPUDetails::AMD(amd_data)))
        } else {
            println!("No rows found of keyword [{}] in column [{}]", keyword, column.to_string());
            Ok(None)
        }
    }

    fn gen_db() {
        let file = format!("{}/amdProcessorInformation_4-12-24.csv", Self::CPU_INFO_FOLDER);

        Self::save_to_database(vec![file]).expect("Unable to save data to database");
    }
}

impl crate::cpu::private::Database for AMDData {
    const DATABASE: &'static str = "res/db/cpu.db";
    const CPU_INFO_FOLDER: &'static str = "res/cpu/amd/AMDCpuData";
    fn save_to_database(files: Vec<String>) -> Result<(), rusqlite::Error> {
        // Initialise connection
        let mut conn = Connection::open(Self::DATABASE)?;

        // Create table (same as before)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS amd_cpus (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            family TEXT,
            series TEXT,
            formFactor TEXT,
            cores INTEGER,
            threads INTEGER,
            boost_clock INTEGER,
            base_clock INTEGER,
            L1Cache TEXT,
            L2Cache TEXT,
            L3Cache TEXT,
            DefaultTDP TEXT,
            AMDConfigurableTDP TEXT,
            lithography TEXT,
            overclocking_enabled BOOLEAN,
            cpu_socket TEXT,
            PIB TEXT,
            MPK TEXT,
            recommended_cooler TEXT,
            operating_temperature_max INTEGER,
            launch_date TEXT,
            os_support TEXT,
            PCI_Express_version TEXT,
            system_memory_type TEXT,
            memory_channels INTEGER,
            system_memory_specification TEXT,
            graphics_model TEXT,
            graphics_cores INTEGER,
            graphics_frequency INTEGER,
            AMD_RyzenAIEnabled BOOLEAN,
            product_id_boxed TEXT,
            product_id_tray TEXT,
            product_id_mpk TEXT,
            supported_technologies TEXT
        )",
            [],
        )?;

        let mut amd_items: Vec<AMDData> = Vec::new();

        for file in files {
            let file_content = match fs::read_to_string(&file) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file {}: {}", file, e);
                    continue;
                }
            };

            for line in file_content.lines().skip(1) {
                let mut fields = Self::split_csv_line(line);

                // Pad fields with "N/A" or default values if insufficient
                if fields.len() < 34 {
                    let mut padded_fields = vec!["N/A".to_string(); 34];
                    for (i, field) in fields.iter().enumerate() {
                        padded_fields[i] = field.to_string();
                    }
                    fields = padded_fields;
                }

                // Parse form factor
                let form_factor = match (fields[3].to_lowercase().contains("laptops"),
                                         fields[3].to_lowercase().contains("desktops")) {
                    (true, false) => FormFactor::Laptops,
                    (false, true) => FormFactor::Desktops,
                    _ => FormFactor::Desktops, // Default to Desktops if unrecognized
                };

                // Parse cores and threads
                let cores = fields[4].parse::<usize>().unwrap_or(0);
                let threads = fields[5].parse::<usize>().unwrap_or(0);

                // Parse boost and base clock (removing "Up to" and "MHz" if present)
                let boost_clock = Self::parse_clock_speed(&fields[6]);
                let base_clock = Self::parse_clock_speed(&fields[7]);

                // Parse graphics
                let graphics = Graphics {
                    model: fields[26].to_string(),
                    cores: fields[27].parse::<usize>().unwrap_or(0),
                    frequency: fields[28].parse::<usize>().unwrap_or(0),
                };

                // Parse OS support
                let os_support: Vec<String> = fields[21]
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                // Parse product ID
                let product_id = ProductID {
                    boxed: if fields[30].is_empty() { None } else { Some(fields[30].to_string()) },
                    tray: if fields[31].is_empty() { None } else { Some(fields[31].to_string()) },
                    mpk: if fields[32].is_empty() { None } else { Some(fields[32].to_string()) },
                };

                // Parse supported technologies
                let supported_technologies: Vec<String> = if !fields[33].is_empty() {
                    fields[33].split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                } else {
                    Vec::new()
                };

                let amd_item = AMDData {
                    name: fields[0].to_string(),
                    family: fields[1].to_string(),
                    series: fields[2].to_string(),
                    formFactor: form_factor,
                    cores,
                    threads,
                    boost_clock,
                    base_clock,
                    L1Cache: fields[11].to_string(),
                    L2Cache: fields[8].to_string(),
                    L3Cache: fields[9].to_string(),
                    DefaultTDP: fields[10].to_string(),
                    AMDConfigurableTDP: fields[12].to_string(),
                    lithography: fields[13].to_string(),
                    overclocking_enabled: match fields[14].to_lowercase().as_str() {
                        "yes" => Some(true),
                        "no" => Some(false),
                        _ => None,
                    },
                    cpu_socket: fields[15].to_string(),
                    PIB: fields[16].to_string(),
                    MPK: fields[18].to_string(),
                    recommended_cooler: if fields[17].is_empty() { None } else { Some(fields[17].to_string()) },
                    operating_temperature_max: fields[19].parse::<usize>().unwrap_or(0),
                    launch_date: fields[20].to_string(),
                    os_support,
                    PCI_Express_version: fields[22].to_string(),
                    system_memory_type: fields[23].to_string(),
                    memory_channels: fields[24].parse::<usize>().unwrap_or(0),
                    system_memory_specification: fields[25].to_string(),
                    graphics,
                    AMD_RyzenAIEnabled: match fields[29].to_lowercase().as_str() {
                        "available" => Some(true),
                        "" => None,
                        _ => Some(false),
                    },
                    product_id,
                    supported_technologies,
                };

                amd_items.push(amd_item);
            }
        }

        // Prepare SQL statement for insertion
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO amd_cpus (
                name, family, series, formFactor, cores, threads, 
                boost_clock, base_clock, L1Cache, L2Cache, L3Cache, 
                DefaultTDP, AMDConfigurableTDP, lithography, 
                overclocking_enabled, cpu_socket, PIB, MPK, 
                recommended_cooler, operating_temperature_max, 
                launch_date, os_support, PCI_Express_version, 
                system_memory_type, memory_channels, 
                system_memory_specification, graphics_model, 
                graphics_cores, graphics_frequency, 
                AMD_RyzenAIEnabled, product_id_boxed, 
                product_id_tray, product_id_mpk, 
                supported_technologies
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, 
                ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, 
                ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, 
                ?30, ?31, ?32, ?33, ?34
            )"
            )?;

            for item in &amd_items {
                stmt.execute(params![
                item.name,
                item.family,
                item.series,
                format!("{:?}", item.formFactor),
                item.cores,
                item.threads,
                item.boost_clock,
                item.base_clock,
                item.L1Cache,
                item.L2Cache,
                item.L3Cache,
                item.DefaultTDP,
                item.AMDConfigurableTDP,
                item.lithography,
                item.overclocking_enabled,
                item.cpu_socket,
                item.PIB,
                item.MPK,
                item.recommended_cooler,
                item.operating_temperature_max,
                item.launch_date,
                item.os_support.join(", "),
                item.PCI_Express_version,
                item.system_memory_type,
                item.memory_channels,
                item.system_memory_specification,
                item.graphics.model,
                item.graphics.cores,
                item.graphics.frequency,
                item.AMD_RyzenAIEnabled,
                item.product_id.boxed,
                item.product_id.tray,
                item.product_id.mpk,
                item.supported_technologies.join(", ")
            ])?;
            }
        }
        tx.commit()?;
        println!("Successfully saved items to database");
        dbg!(amd_items.len());

        Ok(())
    }
}