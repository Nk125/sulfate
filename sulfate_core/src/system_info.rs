#[derive(Debug)]
pub struct OperatingSystemInfo {
    pub name: String,
    pub hwid: Option<String>,
    pub version: Option<String>,
    pub kernel: String,
    pub disks: Vec<DiskInfo>,
    pub ram: RamInfo,
    pub cpu: CpuInfo,
    pub motherboard: Option<String>,
    pub product_info: ProductInfo,
}

#[derive(Debug)]
pub struct ProductInfo {
    pub name: String,
    pub version: String,
    pub family: String,
    pub vendor: String,
    pub serial_number: String,
}

#[derive(Debug)]
pub struct DiskInfo {
    pub available_capacity_in_mb: u64,
    pub total_capacity_in_mb: u64,
    pub mount_path: String,
    pub name: String,
    pub disk_type: String,
    pub is_removable: bool,
    pub is_read_only: bool,
}

#[derive(Debug)]
pub struct RamInfo {
    pub installed_ram_in_mb: u64,
    pub used_ram_in_mb: u64,
}

#[derive(Debug)]
pub struct CpuInfo {
    pub arch: String,
    pub branding: String,
}
