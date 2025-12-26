use sulfate_core::system_info::*;
use sysinfo::{Disk, Disks, Motherboard, Product, System};

const BYTES_IN_A_MEGABYTE: u64 = 1000 * 1000;

fn format_motherboard_info(motherboard: Motherboard) -> String {
    format!(
        "{} {} {}",
        motherboard.vendor_name().unwrap_or("Unknown Vendor".into()),
        motherboard.name().unwrap_or("Unknown Name".into()),
        motherboard.serial_number().unwrap_or("Unknown S/N".into())
    )
}

fn system_characteristics() -> (CpuInfo, RamInfo) {
    let system = System::new_all();
    let cpus = system.cpus();
    let cpu_branding = cpus
        .first()
        .map(|cpu| {
            format!(
                "Brand: {} Name: {} Vendor: {} Freq: {}",
                cpu.brand(),
                cpu.name(),
                cpu.vendor_id(),
                cpu.frequency(),
            )
        })
        .unwrap_or("Unknown CPU".into());
    let cpu_arch = System::cpu_arch();
    let cpu_cores = sysinfo::System::physical_core_count();
    let cpu_logical_cores = cpus.len();

    let installed_ram_in_mb = system.total_memory() / BYTES_IN_A_MEGABYTE;
    let used_ram_in_mb = system.used_memory() / BYTES_IN_A_MEGABYTE;

    (
        CpuInfo {
            arch: cpu_arch,
            branding: cpu_branding,
            cores: cpu_cores,
            logical_cores: cpu_logical_cores,
        },
        RamInfo {
            installed_ram_in_mb,
            used_ram_in_mb,
        },
    )
}

fn disk_info_adapter(disk: &Disk) -> DiskInfo {
    let available_capacity_in_mb = disk.available_space() / BYTES_IN_A_MEGABYTE;
    let total_capacity_in_mb = disk.total_space() / BYTES_IN_A_MEGABYTE;

    let mount_path = disk.mount_point().to_string_lossy().to_string();
    let name = disk.name().to_string_lossy().to_string();

    let filesystem = disk.file_system().to_string_lossy().to_string();

    DiskInfo {
        available_capacity_in_mb,
        total_capacity_in_mb,
        mount_path,
        name,
        filesystem,
        disk_type: disk.kind().to_string(),
        is_removable: disk.is_removable(),
        is_read_only: disk.is_read_only(),
    }
}

fn disks_info() -> Vec<DiskInfo> {
    Disks::new_with_refreshed_list()
        .list()
        .into_iter()
        .map(disk_info_adapter)
        .collect()
}

fn product_info() -> ProductInfo {
    let name = Product::name().unwrap_or("Unknown Product Name".into());
    let version = Product::version().unwrap_or("Unknown Product Version".into());
    let family = Product::family().unwrap_or("Unknown Product Family".into());
    let vendor = Product::vendor_name().unwrap_or("Unknown Product Vendor".into());
    let serial_number = Product::serial_number().unwrap_or("Unknown Product S/N".into());

    ProductInfo {
        name,
        version,
        family,
        vendor,
        serial_number,
    }
}

pub fn get_os_info() -> OperatingSystemInfo {
    let name = System::name().unwrap_or("Unknown".into());
    let hwid = Product::uuid();
    let version = System::long_os_version();
    let kernel = System::kernel_long_version();

    let motherboard = Motherboard::new().map(format_motherboard_info);

    let (cpu, ram) = system_characteristics();

    OperatingSystemInfo {
        name,
        hwid,
        version,
        kernel,
        motherboard,
        cpu,
        ram,
        disks: disks_info(),
        product_info: product_info(),
    }
}
