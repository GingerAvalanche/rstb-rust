use roead::aamp::ParameterIO;

use crate::Endian;
use super::cpp_classes::{agl, sead, LifeCondition::LifeCondition};

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<LifeCondition<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<LifeCondition<u64>>();

const OVERHEAD_WIIU: usize = 0x88;
const OVERHEAD_NX: usize = 0x0;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size: usize = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + OVERHEAD_NX,
    };
    let safestring_size = match endian {
        Endian::Big => size_of::<agl::Parameter<u32, sead::SafeString<u32>>>() + 0x2,
        Endian::Little => size_of::<agl::Parameter<u64, sead::SafeString<u64>>>(),
    };

    let a = ParameterIO::from_binary(bytes).ok()?;

    total_size += a.param_root.objects.iter()
        .filter_map(|(k, v)| {
            match k.hash() {
                3730121527 |
                1959350547 |
                1390640162 |
                1118737566 => Some(v.0.len()),
                _ => None,
            }
        })
        .sum::<usize>() * safestring_size;

    Some(total_size as u32)
}
