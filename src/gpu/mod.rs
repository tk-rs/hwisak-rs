use log::{debug, warn};
use wgpu::{self, Backends, DeviceType};

#[derive(Debug)]
pub enum BackendType {
    GL,
    VULKAN,
    DX12,
    METAL,
}

#[derive(Debug)]
pub struct BackendInfo {
    backend: BackendType,
    supported: bool,
    driver: Option<String>,
    driver_info: Option<String>,
}

#[derive(Debug)]
pub struct GPUDetails {
    name: String,
    vendor: u32,
    device: u32,
    device_type: DeviceType,
    backend_details: Vec<BackendInfo>,
}

impl GPUDetails {
    pub fn fetch() -> Option<Self> {  // Changed return type from Option<Vec<Self>> to Option<Self>
        debug!("Fetching graphics information...");

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        // Take first adapter only
        let adapters: Vec<_> = instance.enumerate_adapters(Backends::all()).into_iter().collect();
        if adapters.is_empty() {
            warn!("No GPU adapters found");
            return None;
        }

        // Use first adapter's info
        let primary_adapter = &adapters[0];
        let info = primary_adapter.get_info();
        let mut backend_details = Vec::new();

        let backends = vec![
            (BackendType::VULKAN, wgpu::Backend::Vulkan),
            (BackendType::DX12, wgpu::Backend::Dx12),
            (BackendType::GL, wgpu::Backend::Gl),
            (BackendType::METAL, wgpu::Backend::Metal),
        ];

        for (backend_type, wgpu_backend) in backends {
            let adapter_for_backend = adapters.iter()
                .find(|a| a.get_info().backend == wgpu_backend);

            backend_details.push(BackendInfo {
                backend: backend_type,
                supported: adapter_for_backend.is_some(),
                driver: adapter_for_backend.map(|a| a.get_info().driver.clone()),
                driver_info: adapter_for_backend.map(|a| a.get_info().driver_info.clone()),
            });
        }

        debug!("Graphics information gathered successfully");
        Some(Self {
            name: info.name,
            vendor: info.vendor,
            device: info.device,
            device_type: info.device_type,
            backend_details,
        })
    }

    fn create_empty() -> Self {
        Self {
            name: String::from("No GPU detected"),
            vendor: 0,
            device: 0,
            device_type: DeviceType::Other,
            backend_details: Vec::new(),
        }
    }

    pub fn is_discrete(&self) -> bool {
        matches!(self.device_type, DeviceType::DiscreteGpu)
    }

    pub fn get_vendor_name(&self) -> &str {
        match self.vendor {
            0x10DE => "NVIDIA",
            0x1002 => "AMD",
            0x8086 => "Intel",
            _ => "Unknown",
        }
    }
}