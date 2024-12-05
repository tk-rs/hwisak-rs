#[derive(Debug)]
pub struct OSDetails {
    os_type: String,
    version: String,
    edition: String,
    codename: String,
    bitness: String,
    architecture: String,
}

impl OSDetails {
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