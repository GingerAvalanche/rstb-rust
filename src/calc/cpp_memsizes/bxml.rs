use roead::aamp::ParameterIO;

use crate::Endian;
use super::cpp_classes::ActorLink::ActorLink;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<ActorLink<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<ActorLink<u64>>();

const TAG_SIZE: usize = std::mem::size_of::<u32>();

const BASE_OVERHEAD_WIIU: usize = 0x34;
const BASE_OVERHEAD_NX: usize = 0x20;
const TAG_OVERHEAD_WIIU: usize = 0x10;
const TAG_OVERHEAD_NX: usize = 0x20;
const DISCREPANCY_OVERHEAD_WIIU: usize = 0x0;
const DISCREPANCY_OVERHEAD_NX: usize = 0x4;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size: usize = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + BASE_OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + BASE_OVERHEAD_NX,
    };
    let a = ParameterIO::from_binary(bytes).ok()?;
    let (tag_overhead, discrepancy_overhead);
    match endian {
        Endian::Big => {
            tag_overhead = TAG_OVERHEAD_WIIU;
            discrepancy_overhead = DISCREPANCY_OVERHEAD_WIIU;
        }
        Endian::Little => {
            tag_overhead = TAG_OVERHEAD_NX;
            discrepancy_overhead = DISCREPANCY_OVERHEAD_NX;
        }
    }

    if let Some(tags) = a.param_root.objects.get("Tags") {
        total_size += tag_overhead + TAG_SIZE * tags.len();
    }

    total_size += discrepancy_overhead;

    Some(total_size as u32)
}
