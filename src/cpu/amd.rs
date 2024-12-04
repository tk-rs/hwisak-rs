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
    fn parse_csv_line(line: &str) -> AMDData {
        let fields: Vec<&str> = line.split(',').collect();
    
        // Helper function to clean and parse integers
        fn parse_int(s: &str) -> usize {
            s.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap_or(0)
        }
    
        // Helper function for boolean options
        fn parse_bool_option(s: &str) -> Option<bool> {
            match s.trim().to_lowercase().as_str() {
                "yes" => Some(true),
                "no" => Some(false),
                _ => None
            }
        }

        fn parse_form_factors(input: &str) -> FormFactor {
            let input = input.trim().to_lowercase();
        
            if input.contains("laptop") || input.contains("notebook") {
                FormFactor::Laptops
            } else if input.contains("1l") || input.contains("mini") || input.contains("tiny") {
                FormFactor::TinyDesktops
            } else if input.contains("boxed processor") || input.contains("pib") {
                FormFactor::BoxedProcessor
            } else if input.contains("mobile workstation") {
                FormFactor::MobileWorkstations
            } else if input.contains("handheld") || input.contains("portable") {
                FormFactor::Handheld
            } else if input.contains("desktop") || input.is_empty() {
                FormFactor::Desktops  // Default case
            } else {
                println!("Fallback for fn parse_form_factors");
                FormFactor::Desktops  // Fallback
            }
        }
    
        AMDData {
            name: fields.get(0).unwrap_or(&"Unknown").trim().to_string(),
            family: fields.get(1).unwrap_or(&"Unknown").trim().to_string(),
            series: fields.get(2).unwrap_or(&"Unknown").trim().to_string(),
            formFactor: parse_form_factors(fields.get(3).unwrap_or(&"")),
            cores: parse_int(fields.get(4).unwrap_or(&"0")),
            threads: parse_int(fields.get(5).unwrap_or(&"0")),
            boost_clock: parse_int(fields.get(6).unwrap_or(&"0")),
            base_clock: parse_int(fields.get(7).unwrap_or(&"0")),
            L1Cache: fields.get(8).unwrap_or(&"").trim().to_string(),
            L2Cache: fields.get(9).unwrap_or(&"").trim().to_string(),
            L3Cache: fields.get(10).unwrap_or(&"").trim().to_string(),
            DefaultTDP: fields.get(11).unwrap_or(&"").trim().to_string(),
            AMDConfigurableTDP: fields.get(12).unwrap_or(&"").trim().to_string(),
            lithography: fields.get(13).unwrap_or(&"").trim().to_string(),
            overclocking_enabled: parse_bool_option(fields.get(14).unwrap_or(&"")),
            cpu_socket: fields.get(15).unwrap_or(&"").trim().to_string(),
            PIB: fields.get(16).unwrap_or(&"").trim().to_string(),
            MPK: fields.get(17).unwrap_or(&"").trim().to_string(),
            recommended_cooler: fields.get(18)
                .filter(|&s| !s.trim().is_empty())
                .map(|s| s.trim().to_string()),
            operating_temperature_max: parse_int(fields.get(19).unwrap_or(&"0")),
            launch_date: fields.get(20).unwrap_or(&"").trim().to_string(),
            os_support: fields.get(21)
                .map(|s| s.split(',').map(|os| os.trim().to_string()).collect())
                .unwrap_or_else(|| vec![]),
            PCI_Express_version: fields.get(22).unwrap_or(&"").trim().to_string(),
            system_memory_type: fields.get(23).unwrap_or(&"").trim().to_string(),
            memory_channels: parse_int(fields.get(24).unwrap_or(&"0")),
            system_memory_specification: fields.get(25).unwrap_or(&"").trim().to_string(),
            graphics: Graphics {
                model: fields.get(26).unwrap_or(&"").trim().to_string(),
                cores: parse_int(fields.get(27).unwrap_or(&"0")),
                frequency: parse_int(fields.get(28).unwrap_or(&"0")),
            },
            AMD_RyzenAIEnabled: parse_bool_option(fields.get(29).unwrap_or(&"")),
            product_id: ProductID {
                boxed: fields.get(30).filter(|&s| !s.trim().is_empty()).map(|s| s.trim().to_string()),
                tray: fields.get(31).filter(|&s| !s.trim().is_empty()).map(|s| s.trim().to_string()),
                mpk: fields.get(32).filter(|&s| !s.trim().is_empty()).map(|s| s.trim().to_string()),
            },
            supported_technologies: fields.get(33)
                .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
                .unwrap_or_else(|| vec![]),
        }
    }
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
        let mut conn = Connection::open(Self::DATABASE)?;
    
        conn.execute(
            "CREATE TABLE IF NOT EXISTS amd_cpus (
                name TEXT PRIMARY KEY,
                family TEXT,
                series TEXT,
                form_factor TEXT,
                cores TEXT,
                threads TEXT,
                boost_clock TEXT,
                base_clock TEXT,
                L1Cache TEXT,
                L2Cache TEXT,
                L3Cache TEXT,
                DefaultTDP TEXT,
                AMDConfigurableTDP TEXT,
                lithography TEXT,
                overclocking_enabled TEXT,
                cpu_socket TEXT,
                PIB TEXT,
                MPK TEXT,
                recommended_cooler TEXT,
                operating_temperature_max TEXT,
                launch_date TEXT,
                os_support TEXT,
                PCI_Express_version TEXT,
                system_memory_type TEXT,
                memory_channels TEXT,
                system_memory_specification TEXT,
                graphics_model TEXT,
                graphics_cores TEXT,
                graphics_frequency TEXT,
                AMD_RyzenAIEnabled TEXT,
                product_id_boxed TEXT,
                product_id_tray TEXT,
                product_id_mpk TEXT,
                supported_technologies TEXT
            )",
            [],
        )?;
    
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(r#"
            INSERT OR REPLACE INTO amd_cpus VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
                ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30,
                ?31, ?32, ?33, ?34
            )
            "#)?;
    
            for file in files {
                let content = fs::read_to_string(&file).map_err(|e| {
                    eprintln!("Error reading file {}: {}", file, e);
                    rusqlite::Error::InvalidParameterName(e.to_string())
                })?;
    
                for line in content.lines().skip(1) {
                    let amd_data = Self::parse_csv_line(line);
                    
                    stmt.execute(params![
                        amd_data.name,
                        amd_data.family,
                        amd_data.series,
                        format!("{:?}", amd_data.formFactor),
                        amd_data.cores,
                        amd_data.threads,
                        amd_data.boost_clock,
                        amd_data.base_clock,
                        amd_data.L1Cache,
                        amd_data.L2Cache,
                        amd_data.L3Cache,
                        amd_data.DefaultTDP,
                        amd_data.AMDConfigurableTDP,
                        amd_data.lithography,
                        amd_data.overclocking_enabled.map_or("N/A".to_string(), |b| b.to_string()),
                        amd_data.cpu_socket,
                        amd_data.PIB,
                        amd_data.MPK,
                        amd_data.recommended_cooler.unwrap_or_default(),
                        amd_data.operating_temperature_max,
                        amd_data.launch_date,
                        amd_data.os_support.join(","),
                        amd_data.PCI_Express_version,
                        amd_data.system_memory_type,
                        amd_data.memory_channels,
                        amd_data.system_memory_specification,
                        amd_data.graphics.model,
                        amd_data.graphics.cores,
                        amd_data.graphics.frequency,
                        amd_data.AMD_RyzenAIEnabled.map_or("N/A".to_string(), |b| b.to_string()),
                        amd_data.product_id.boxed.unwrap_or_default(),
                        amd_data.product_id.tray.unwrap_or_default(),
                        amd_data.product_id.mpk.unwrap_or_default(),
                        amd_data.supported_technologies.join(",")
                    ])?;
                }
            }
        }
        tx.commit()?;
    
        Ok(())
    }
}