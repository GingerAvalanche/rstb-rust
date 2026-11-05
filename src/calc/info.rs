use crate::calc::cpp_memsizes::{
    baiprog,
    baniminfo,
    baslist,
    bchemical,
    bdrop,
    bgparamlist,
    blifecondition,
    bmodellist,
    bphysics,
    bphyssb,
    brecipe,
    brgconfiglist,
    bshop,
    bxml
};
use crate::calc::cpp_memsizes::cpp_classes::{
    ActorLink::ActorLink,
    AIProgram::AIProgram,
    AnimationInfo::AnimInfo,
    ASList::ASList,
    Chemical::Chemical,
    DropTable::Drop,
    GParamList::GParamList,
    LifeCondition::LifeCondition,
    ModelList::ModelList,
    Physics::Physics,
    RagdollConfigList::RagdollConfigList,
    Recipe::Recipe,
    ShopData::Shop,
    SupportBone::SupportBoneResource,
};
use crate::Endian;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParseSize {
    Simple(u32),
    Complex(Option<u32>),
}

pub(crate) fn get_factory_info<S: AsRef<str>>(ext: S, endian: Endian, bytes: &[u8]) -> (u32, isize, ParseSize) {
    match ext.as_ref() {
        "sarc" | "pack" | "bactorpack" | "bmodelsh" | "beventpack" | "stera" | "stats" => {
            match endian {
                Endian::Little => (0x68, 0x80, ParseSize::Simple(0)),
                Endian::Big => (0x3c, 0x80, ParseSize::Simple(0)),
            }
        }
        "Tex.bfres" | "Tex1.bfres" | "Tex2.bfres" => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x20, 0x4, ParseSize::Simple(0)),
            }
        }
        "bfres" => {
            match endian {
                Endian::Little => (0x1a8, 0x1000, ParseSize::Complex(None)),
                Endian::Big => (0x13c, 0x1000, ParseSize::Complex(None)),
            }
        }
        "bcamanim" => {
            match endian {
                Endian::Little => (0x50, 0x2000, ParseSize::Complex(None)),
                Endian::Big => (0x2c, 0x2000, ParseSize::Complex(None)),
            }
        }
        "batpl" | "bnfprl" => {
            match endian {
                Endian::Little => (0x40, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x24, 0x4, ParseSize::Simple(0)),
            }
        }
        "bplacement" => {
            match endian {
                Endian::Little => (0x48, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0)),
            }
        }
        "hks" | "lua" => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0)),
            }
        }
        "bactcapt" => {
            match endian {
                Endian::Little => (0x538, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x3b4, 0x4, ParseSize::Simple(0)),
            }
        }
        "bitemico" => {
            match endian {
                Endian::Little => (0x60, 0x2000, ParseSize::Simple(0)),
                Endian::Big => (0xd0, 0x2000, ParseSize::Simple(0)),
            }
        }
        "jpg" => {
            match endian {
                Endian::Little => (0x80, 0x2000, ParseSize::Simple(0)),
                Endian::Big => (0x174, 0x2000, ParseSize::Simple(0)),
            }
        }
        "bmaptex" => {
            match endian {
                Endian::Little => (0x60, 0x2000, ParseSize::Simple(0)),
                Endian::Big => (0xd0, 0x2000, ParseSize::Simple(0)),
            }
        }
        "bstftex" | "bmapopen" | "breviewtex" => {
            match endian {
                Endian::Little => (0x60, 0x2000, ParseSize::Simple(0)),
                Endian::Big => (0xd0, 0x2000, ParseSize::Simple(0)),
            }
        }
        "bgdata" => {
            match endian {
                Endian::Little => (0x140, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0xcc, 0x4, ParseSize::Simple(0)),
            }
        }
        "bgsvdata" => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0)),
            }
        }
        "hknm2" => {
            match endian {
                Endian::Little => (0x48, 0x8, ParseSize::Complex(Some(0x248))),
                Endian::Big => (0x28, 0x4, ParseSize::Complex(Some(0x174))),
            }
        }
        "bmscdef" => {
            match endian {
                Endian::Little => (0x2a8, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x1fc, 0x4, ParseSize::Complex(None)),
            }
        }
        "bars" => {
            match endian {
                Endian::Little => (0xb0, 0x80, ParseSize::Complex(None)),
                Endian::Big => (0x84, 0x80, ParseSize::Complex(None)),
            }
        }
        "bxml" => {
            match endian {
                Endian::Little => (
                    size_of::<ActorLink<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bxml::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<ActorLink<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bxml::parse_size(bytes, endian))
                ),
            }
        }
        "bgparamlist" => {
            match endian {
                Endian::Little => (
                    size_of::<GParamList<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bgparamlist::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<GParamList<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bgparamlist::parse_size(bytes, endian))
                ),
            }
        }
        "bmodellist" => {
            match endian {
                Endian::Little => (
                    size_of::<ModelList<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bmodellist::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<ModelList<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bmodellist::parse_size(bytes, endian))
                ),
            }
        }
        "baslist" => {
            match endian {
                Endian::Little => (
                    size_of::<ASList<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(baslist::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<ASList<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(baslist::parse_size(bytes, endian))
                ),
            }
        }
        "baiprog" => {
            match endian {
                Endian::Little => (
                    size_of::<AIProgram<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(baiprog::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<AIProgram<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(baiprog::parse_size(bytes, endian))
                ),
            }
        }
        "bphysics" => {
            match endian {
                Endian::Little => (
                    size_of::<Physics<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bphysics::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<Physics<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bphysics::parse_size(bytes, endian))
                ),
            }
        }
        "bchemical" => {
            match endian {
                Endian::Little => (
                    size_of::<Chemical<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bchemical::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<Chemical<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bchemical::parse_size(bytes, endian))
                ),
            }
        }
        "bas" => {
            match endian {
                Endian::Little => (0x3c8, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x2d0, 0x4, ParseSize::Complex(None)),
            }
        }
        "batcllist" => {
            match endian {
                Endian::Little => (0x3f0, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x2e4, 0x4, ParseSize::Complex(None)),
            }
        }
        "batcl" => {
            match endian {
                Endian::Little => (0x428, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x344, 0x4, ParseSize::Complex(None)),
            }
        }
        "baischedule" => {
            match endian {
                Endian::Little => (0x2b8, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x244, 0x4, ParseSize::Simple(0x18)),
            }
        }
        "bdmgparam" => {
            match endian {
                Endian::Little => (0x11d0, 0x8, ParseSize::Simple(0x7f8)),
                Endian::Big => (0x9f0, 0x4, ParseSize::Simple(0x40c)),
            }
        }
        "brgconfiglist" => {
            match endian {
                Endian::Little => (
                    size_of::<RagdollConfigList<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(brgconfiglist::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<RagdollConfigList<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(brgconfiglist::parse_size(bytes, endian))
                ),
            }
        }
        "brgconfig" => {
            match endian {
                Endian::Little => (0x42d8, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x2acc, 0x4, ParseSize::Simple(0x10)),
            }
        }
        "brgbw" => {
            match endian {
                Endian::Little => (0x2c0, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x248, 0x4, ParseSize::Complex(None)),
            }
        }
        "bawareness" => {
            match endian {
                Endian::Little => (0xb38, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x70c, 0x4, ParseSize::Simple(0x10)),
            }
        }
        "bdrop" => {
            match endian {
                Endian::Little => (
                    size_of::<Drop<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bdrop::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<Drop<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bdrop::parse_size(bytes, endian))
                ),
            }
        }
        "bshop" => {
            match endian {
                Endian::Little => (
                    size_of::<Shop<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bshop::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<Shop<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bshop::parse_size(bytes, endian))
                ),
            }
        }
        "brecipe" => {
            match endian {
                Endian::Little => (
                    size_of::<Recipe<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(brecipe::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<Recipe<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(brecipe::parse_size(bytes, endian))
                ),
            }
        }
        "blod" => {
            match endian {
                Endian::Little => (0x3c0, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x2cc, 0x4, ParseSize::Simple(0x10)),
            }
        }
        "bbonectrl" => {
            match endian {
                Endian::Little => (0x8d0, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x564, 0x4, ParseSize::Complex(None)),
            }
        }
        "blifecondition" => {
            match endian {
                Endian::Little => (
                    size_of::<LifeCondition<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(blifecondition::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<LifeCondition<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(blifecondition::parse_size(bytes, endian))
                ),
            }
        }
        "bumii" => {
            match endian {
                Endian::Little => (0x2b8, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x244, 0x4, ParseSize::Simple(0x18)),
            }
        }
        "baniminfo" => {
            match endian {
                Endian::Little => (
                    size_of::<AnimInfo<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(baniminfo::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<AnimInfo<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(baniminfo::parse_size(bytes, endian))
                ),
            }
        }
        "byaml" => {
            match endian {
                Endian::Little => (0x20, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0)),
            }
        }
        "byml" => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0x28)),
                Endian::Big => (0x20, 0x4, ParseSize::Simple(0x3c)),
            }
        }
        "bassetting" => {
            match endian {
                Endian::Little => (0x260, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x1d8, 0x4, ParseSize::Complex(None)),
            }
        }
        "hkrb" => {
            match endian {
                Endian::Little => (0x20, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0x8)),
            }
        }
        "hkrg" => {
            match endian {
                Endian::Little => (0x20, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x14, 0x4, ParseSize::Simple(0x8)),
            }
        }
        "bphyssb" => {
            match endian {
                Endian::Little => (
                    size_of::<SupportBoneResource<u64>>() as u32,
                    0x8,
                    ParseSize::Complex(bphyssb::parse_size(bytes, endian))
                ),
                Endian::Big => (
                    size_of::<SupportBoneResource<u32>>() as u32,
                    0x4,
                    ParseSize::Complex(bphyssb::parse_size(bytes, endian))
                ),
            }
        }
        "hkcl" => {
            match endian {
                Endian::Little => (0xe8, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0xb8, 0x4, ParseSize::Complex(None)),
            }
        }
        "hksc" => {
            match endian {
                Endian::Little => (0x140, 0x8, ParseSize::Complex(Some(0x9ac0))),
                Endian::Big => (0xe8, 0x4, ParseSize::Complex(Some(0x73e4))),
            }
        }
        "hktmrb" => {
            match endian {
                Endian::Little => (0x48, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x28, 0x4, ParseSize::Complex(None)),
            }
        }
        "brgcon" => {
            match endian {
                Endian::Little => (0x48, 0x8, ParseSize::Complex(None)),
                Endian::Big => (0x28, 0x4, ParseSize::Complex(None)),
            }
        }
        "esetlist" => {
            match endian {
                Endian::Little => (0x38, 0x4000, ParseSize::Simple(0)),
                Endian::Big => (0x20, 0x4000, ParseSize::Simple(0)),
            }
        }
        "bdemo" => {
            match endian {
                Endian::Little => (0xb20, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x6cc, 0x4, ParseSize::Simple(0x10)),
            }
        }
        "bfevfl" => {
            match endian {
                Endian::Little => (0x40, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x24, 0x4, ParseSize::Simple(0x38)),
            }
        }
        "bfevtm" => {
            match endian {
                Endian::Little => (0x40, 0x8, ParseSize::Simple(0x18)),
                Endian::Big => (0x24, 0x4, ParseSize::Simple(0x38)),
            }
        }
        "mubin" => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0x20)),
                Endian::Big => (0x20, 0x4, ParseSize::Simple(0x1C)),
            }
        }
        _ => {
            match endian {
                Endian::Little => (0x38, 0x8, ParseSize::Simple(0)),
                Endian::Big => (0x20, 0x4, ParseSize::Simple(0)),
            }
        }
    }
}
