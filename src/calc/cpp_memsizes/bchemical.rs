use std::mem::size_of;

use roead::aamp::ParameterIO;

use super::cpp_classes::Chemical::*;
use crate::Endian;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<Chemical<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<Chemical<u64>>();

const OVERHEAD_WIIU: usize = 0x0;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX,
    };

    let a = ParameterIO::from_binary(bytes).ok()?;
    let (shape_size, rigid_size): (usize, usize);
    match endian {
        Endian::Big => {
            shape_size = size_of::<Shape<u32>>();
            rigid_size = size_of::<Rigid<u32>>();
        }
        Endian::Little => {
            shape_size = size_of::<Shape<u64>>();
            rigid_size = size_of::<Rigid<u64>>();
        }
    }

    if let Some(root) = a.param_root.lists.get("chemical_root") {
        if let Some(header) = root.objects.get("chemical_header") {
            if let Some(res_shape_num) = header.get("res_shape_num") {
                let num_sets: usize = usize::try_from(res_shape_num.as_u32().ok()?).ok()?;
                total_size += num_sets * (shape_size + rigid_size);
            }
        }
    }
    Some(total_size as u32)
}
