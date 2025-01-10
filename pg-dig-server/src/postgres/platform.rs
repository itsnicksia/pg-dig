use scroll::Endian;

#[cfg(target_endian = "big")]
pub fn get_platform_endian() -> Endian { scroll::BE }

#[cfg(target_endian = "little")]
pub fn get_platform_endianness() -> Endian { scroll::LE }
