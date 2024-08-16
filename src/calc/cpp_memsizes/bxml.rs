use roead::aamp::ParameterIO;

use crate::Endian;
use super::cpp_classes::ActorLink::ActorLink;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<ActorLink<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<ActorLink<u64>>();

const TAG_SIZE: usize = std::mem::size_of::<u32>();

const OVERHEAD_WIIU: usize = 0x64;
const OVERHEAD_NX: usize = 0x44;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size: usize = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + OVERHEAD_NX,
    };
    let a = ParameterIO::from_binary(bytes).ok()?;

    if let Some(tags) = a.param_root.objects.get("Tags") {
        total_size += TAG_SIZE * tags.len();
    }

    Some(total_size as u32)
}
