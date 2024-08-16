use std::mem::size_of;

use roead::aamp::ParameterIO;

use super::cpp_classes::SupportBone::*;
use crate::Endian;

const CLASS_SIZE_WIIU: usize = size_of::<SupportBoneResource<u32>>();
const CLASS_SIZE_NX: usize = size_of::<SupportBoneResource<u64>>();

const OVERHEAD_WIIU: usize = 0x98;
const OVERHEAD_NX: usize = 0x0;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + OVERHEAD_NX,
    };

    let (
        bone_size,
        connection_linear_size,
        connection_curve_size,
        output_single_size
    ): (usize, usize, usize, usize);
    let (
        output_double_size,
        main_bone_size,
        support_bone_size
    ): (usize, usize, usize);
    match endian {
        Endian::Big => {
            bone_size = size_of::<Bone<u32>>();
            connection_linear_size = size_of::<ConnectionLinear<u32>>();
            connection_curve_size = size_of::<ConnectionCurve<u32>>();
            output_single_size = size_of::<OutputSingle<u32>>();
            output_double_size = size_of::<OutputDouble<u32>>();
            main_bone_size = size_of::<MainBone<u32>>();
            support_bone_size = size_of::<SupportBone<u32>>();
        }
        Endian::Little => {
            bone_size = size_of::<Bone<u64>>();
            connection_linear_size = size_of::<ConnectionLinear<u64>>();
            connection_curve_size = size_of::<ConnectionCurve<u64>>();
            output_single_size = size_of::<OutputSingle<u64>>();
            output_double_size = size_of::<OutputDouble<u64>>();
            main_bone_size = size_of::<MainBone<u64>>();
            support_bone_size = size_of::<SupportBone<u64>>();
        }
    }

    let a = ParameterIO::from_binary(bytes).ok()?;
    if let Some(header) = a.param_root.objects.get("support_bone_header") {
        let bone_num: usize = header
            .get("bone_num")?
            .as_int()
            .ok()?;
        total_size += bone_num * bone_size;
        let connection_linear_num: usize = header
            .get("connection_linear_num")?
            .as_int()
            .ok()?;
        total_size += connection_linear_num * connection_linear_size;
        let connection_curve_num: usize = header
            .get("connection_curve_num")?
            .as_int()
            .ok()?;
        total_size += connection_curve_num * connection_curve_size;
        let output_single_num: usize = header
            .get("output_single_num")?
            .as_int()
            .ok()?;
        total_size += output_single_num * output_single_size;
        let output_double_num: usize = header
            .get("output_double_num")?
            .as_int()
            .ok()?;
        total_size += output_double_num * output_double_size;
        let main_bone_num: usize = header
            .get("main_bone_num")?
            .as_int()
            .ok()?;
        total_size += main_bone_num * main_bone_size;
        let support_bone_num: usize = header
            .get("support_bone_num")?
            .as_int()
            .ok()?;
        total_size += support_bone_num * support_bone_size;
    }

    Some(total_size as u32)
}
