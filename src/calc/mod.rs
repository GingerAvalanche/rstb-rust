//! This module handles calculating RSTB values. It can be used for exact
//! calculations on many filetypes or estimations on certain others.
//! The most important distinction is between the `calc` and `estimate`
//! functions. *The `calc` functions only attempt infallible calculations on
//! types where this is supported, and otherwise always return `None`.* By
//! contrast, *the `estimate` functions will perform an infallible calculation
//! on supported types or attempt estimates on supported BFRES and AAMP files*.
//!
//! A full list of supported infallible calculation types:
//! - `sarc` (`pack`, `bactorpack`, `bmodelsh`, `beventpack`, `stera`, `stats`)
//! - `batpl` (`bnfprl`)
//! - `bplacement`
//! - `hks` (`lua`)
//! - `bactcapt`
//! - `bitemico`
//! - `jpg`
//! - `bmaptex`
//! - `bstftex` (`bmapopen`, `breviewtex`)
//! - `bgdata`
//! - `bgsvdata`
//! - `baischedule`
//! - `bdmgparam`
//! - `brgconfig`
//! - `bawareness`
//! - `blod`
//! - `bumii`
//! - `byaml`
//! - `hkrb`
//! - `hkrg`
//! - `esetlist`
//! - `bdemo`
//! - `bfevfl`
//! - `bfevtm`
//!
//! A full list of supported estimation types:
//! - `baiprog`
//! - `bas`
//! - `bphysics`
//! - `baslist`
//! - `bdrop`
//! - `bgparamlist`
//! - `brecipe`
//! - `bshop`
//! - `bxml`
//! - `bfres`
//!
//! **To repeat:** the `calc` functions can only do infallible calculations,
//! and otherwise return `None`. The `estimate` functions will first try the
//! infallible calculation, then try and estimate, and return `None` only if
//! neither works.
//!
//! *A final note*: As mentioned elsewhere, the `endian` parameter should be
//! used as a shorthand for Wii U/Switch more generally. Pass `Endian::Big` for
//! Wii U files, even if they are actually in little endian, and pass
//! `Endian:Little` for Switch files, even if they are actually in big endian.

mod info;

use crate::{Endian, Result};
use info::{get_factory_info, ParseSize};
use std::{io::Read, path::Path};

#[inline]
fn round_32(size: usize) -> u32 {
    ((size as isize + 31) & -32) as u32
}

/// Infallibly calculate an RSTB value from a file on disk, returning `None` if
/// the type is not supported.
pub fn calc_from_file<P: AsRef<Path>>(file: P, endian: Endian) -> Result<Option<u32>> {
    let mut file_handle = std::fs::File::open(file.as_ref())?;
    let mut magic: [u8; 4] = [0; 4];
    file_handle.read_exact(&mut magic)?;
    let size = if &magic == b"Yaz0" {
        file_handle.read_exact(&mut magic)?;
        u32::from_be_bytes(magic) as usize
    } else {
        file_handle.metadata()?.len() as usize
    };
    Ok(calc_from_size_and_name(
        size,
        file.as_ref()
            .file_name()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Path not a file"))?
            .to_str()
            .unwrap(),
        endian,
    ))
}

/// Infallibly calculate an RSTB value from a byte slice and filename,
/// returning `None` if the type is not supported.
pub fn calc_from_slice_and_name<B: AsRef<[u8]>>(
    bytes: B,
    name: &str,
    endian: Endian,
) -> Option<u32> {
    let bytes = bytes.as_ref();
    if bytes.len() < 8 {
        None
    } else if &bytes[0..4] == b"Yaz0" {
        calc_from_size_and_name(
            u32::from_be_bytes(bytes[4..8].try_into().unwrap()) as usize,
            name,
            endian,
        )
    } else {
        calc_from_size_and_name(bytes.len(), name, endian)
    }
}

/// Infallibly calculate an RSTB value from an uncompressed file size and
/// filename, returning `None` if the type is not supported.
pub fn calc_from_size_and_name(filesize: usize, name: &str, endian: Endian) -> Option<u32> {
    calc_or_estimate_from_size_and_name(filesize, name, endian, false)
}

/// Infallibly calculate *or* estimate an RSTB value from a file on disk,
/// returning `None` if the type is not supported.
pub fn estimate_from_file<P: AsRef<Path>>(file: P, endian: Endian) -> Result<Option<u32>> {
    let mut file_handle = std::fs::File::open(file.as_ref())?;
    let mut magic: [u8; 4] = [0; 4];
    file_handle.read_exact(&mut magic)?;
    let size = if &magic == b"Yaz0" {
        file_handle.read_exact(&mut magic)?;
        u32::from_be_bytes(magic) as usize
    } else {
        file_handle.metadata()?.len() as usize
    };
    Ok(estimate_from_size_and_name(
        size,
        file.as_ref()
            .file_name()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Path not a file"))?
            .to_str()
            .unwrap(),
        endian,
    ))
}

/// Infallibly calculate *or* estimate an RSTB value from a byte slice and
/// filename, returning `None` if the type is not supported.
pub fn estimate_from_slice_and_name<B: AsRef<[u8]>>(
    bytes: B,
    name: &str,
    endian: Endian,
) -> Option<u32> {
    let bytes = bytes.as_ref();
    if bytes.len() < 8 {
        None
    } else if &bytes[0..4] == b"Yaz0" {
        estimate_from_size_and_name(
            u32::from_be_bytes(bytes[4..8].try_into().unwrap()) as usize,
            name,
            endian,
        )
    } else {
        estimate_from_size_and_name(bytes.len(), name, endian)
    }
}

/// Infallibly calculate an RSTB value from an uncompressed file size and
/// filename, returning `None` if the type is not supported.
pub fn estimate_from_size_and_name(filesize: usize, name: &str, endian: Endian) -> Option<u32> {
    calc_or_estimate_from_size_and_name(filesize, name, endian, true)
}

fn calc_or_estimate_from_size_and_name(
    filesize: usize,
    name: &str,
    endian: Endian,
    estimate: bool,
) -> Option<u32> {
    if let Some(dot_pos) = name.rfind('.') {
        let rounded = round_32(filesize);
        let raw_ext = &name[dot_pos + 1..];
        let ext = match raw_ext {
            "sarc" => "sarc",
            _ => {
                if let Some(ext) = raw_ext.strip_prefix('s') {
                    ext
                } else {
                    raw_ext
                }
            }
        };
        let (size, parse_size) = get_factory_info(ext, endian);
        match parse_size {
            ParseSize::Simple(parse_size) => Some(match endian {
                Endian::Big => {
                    rounded
                        + 0xe4
                        + size
                        + parse_size
                        + match ext {
                            "beventpack" => 0xe0,
                            "bfevfl" => 0x58,
                            "hkrb" => 40,
                            _ => 0,
                        }
                }
                Endian::Little => rounded + 0x168 + size + parse_size,
            }),
            ParseSize::Complex => {
                if estimate {
                    match ext {
                        "baniminfo" => Some(
                            (rounded as f32 * (if filesize > 36864 { 1.5 } else { 4.0 })) as u32
                                + 0xe4
                                + 0x24c,
                        ),
                        "bfres" => Some(estimate_bfres(filesize, name, endian)),
                        _ => estimate_aamp(filesize, name, endian),
                    }
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

fn estimate_aamp(filesize: usize, name: &str, endian: Endian) -> Option<u32> {
    let mut size = (filesize as f32) * 1.05;
    let ext = &name[name.rfind('.').unwrap() + 1..];
    if ext == "bas" {
        size *= 1.05;
    };
    if ext == "bdmgparam" {
        size = (((-0.0018 * size) + 6.6273) * size) + 500.0
    } else if ext == "bphysics" {
        size = (round_32(size as usize) + 0x4E + 0x324) as f32
            * f32::max(4.0 * (size / 1388.0).floor(), 3.0)
    } else {
        size *= match ext {
            "baiprog" => match size as usize {
                (0..380) => 7.0,
                (380..400) => 6.0,
                (400..450) => 5.5,
                (450..600) => 5.0,
                (600..1_000) => 4.0,
                (1_000..1_750) => 3.5,
                _ => 3.0,
            },
            "bas" => match size as usize {
                (0..100) => 20.0,
                (100..200) => 12.5,
                (200..300) => 10.0,
                (300..600) => 8.0,
                (600..1_500) => 6.0,
                (1_500..2_000) => 5.5,
                (2_000..15_000) => 5.0,
                _ => 4.5,
            },
            "baslist" => match size as usize {
                (0..100) => 15.0,
                (100..200) => 10.0,
                (200..300) => 8.0,
                (300..500) => 6.0,
                (500..800) => 5.0,
                (800..4_000) => 4.0,
                _ => 3.5,
            },
            "bdrop" => match size as usize {
                (0..200) => 8.5,
                (200..250) => 7.0,
                (250..350) => 6.0,
                (350..450) => 5.25,
                (450..850) => 4.5,
                _ => 4.0,
            },
            "bgparamlist" => match size as usize {
                (0..100) => 20.0,
                (100..150) => 12.0,
                (150..250) => 10.0,
                (250..350) => 8.0,
                (350..450) => 7.0,
                _ => 6.0,
            },
            "brecipe" => match size as usize {
                (0..100) => 12.5,
                (100..160) => 8.5,
                (160..200) => 7.5,
                (200..215) => 7.0,
                _ => 6.5,
            },
            "bshop" => match size as usize {
                (0..200) => 7.25,
                (200..400) => 6.0,
                (400..500) => 5.0,
                _ => 4.05,
            },
            "bxml" => match size as usize {
                (0..350) => 6.0,
                (350..450) => 5.0,
                (450..550) => 4.5,
                (550..650) => 4.0,
                (650..800) => 3.5,
                _ => 3.0,
            },
            _ => return None,
        };
    }
    Some(match endian {
        Endian::Big => size,
        Endian::Little => size * 1.5,
    } as u32)
}

fn estimate_bfres(filesize: usize, name: &str, endian: Endian) -> u32 {
    (filesize as f32
        * match endian {
            Endian::Big => {
                if name.contains("Tex") {
                    match filesize {
                        (0..100) => 9.0,
                        (100..2_000) => 7.0,
                        (2_000..3_000) => 5.0,
                        (3_000..4_000) => 4.0,
                        (4_000..8_500) => 3.0,
                        (8_500..12_000) => 2.0,
                        (12_000..17_000) => 1.75,
                        (17_000..30_000) => 1.5,
                        (30_000..45_000) => 1.3,
                        (45_000..100_000) => 1.2,
                        (100_000..150_000) => 1.1,
                        (150_000..200_000) => 1.07,
                        (200_000..250_000) => 1.045,
                        (250_000..300_000) => 1.035,
                        (300_000..600_000) => 1.03,
                        (600_000..1_000_000) => 1.015,
                        (1_000_000..1_800_000) => 1.009,
                        (1_800_000..4_500_000) => 1.005,
                        (4_500_000..6_000_000) => 1.002,
                        _ => 1.0015,
                    }
                } else {
                    match filesize {
                        (0..500) => 7.0,
                        (500..750) => 5.0,
                        (750..1_250) => 4.0,
                        (1_250..2_000) => 3.5,
                        (2_000..400_000) => 2.25,
                        (400_000..600_000) => 2.1,
                        (600_000..1_000_000) => 1.95,
                        (1_000_000..1_500_000) => 1.85,
                        (1_500_000..3_000_000) => 1.66,
                        _ => 1.45,
                    }
                }
            }
            Endian::Little => {
                if name.contains("Tex") {
                    match filesize {
                        (0..10_000) => 2.0,
                        (10_000..30_000) => 1.5,
                        (30_000..50_000) => 1.3,
                        _ => 1.2,
                    }
                } else {
                    match filesize {
                        (0..1_250) => 9.5,
                        (1_250..2_500) => 6.0,
                        (2_500..50_000) => 4.25,
                        (50_000..100_000) => 3.66,
                        (100_000..800_000) => 3.5,
                        (800_000..2_000_000) => 3.15,
                        (2_000_000..3_000_000) => 2.5,
                        (3_000_000..4_000_000) => 1.667,
                        _ => 1.6,
                    }
                }
            }
        }) as u32
}

#[cfg(test)]
mod tests {
    use all_asserts::assert_ge;

    use crate::Endian;
    use std::fs::read;

    #[test]
    fn calc_sizes() {
        assert_eq!(
            super::calc_from_file("test/A-1_Dynamic.smubin", Endian::Big).unwrap(),
            Some(48772)
        );
        assert_eq!(
            super::calc_from_file("test/AirOcta_Tag.sbactorpack", Endian::Little).unwrap(),
            Some(6192)
        );
        let buffer: Vec<u8> = read("test/Enemy_Bokoblin_Gold.bdmgparam").unwrap();
        assert_eq!(
            super::calc_from_slice_and_name(&buffer, "Enemy_Bokoblin_Gold.bdmgparam", Endian::Big),
            Some(5396)
        );
        let buffer: Vec<u8> = read("test/Obj_TreeWhiteBirch_A_01.hkrb").unwrap();
        assert_eq!(
            super::calc_from_slice_and_name(&buffer, "Obj_TreeWhiteBirch_A_01.hkrb", Endian::Big),
            Some(3560)
        );
        let buffer: Vec<u8> = read("test/savedataformat.ssarc").unwrap();
        assert_eq!(
            super::calc_from_slice_and_name(&buffer, "savedataformat.ssarc", Endian::Big),
            Some(2_801_216)
        );
    }

    #[test]
    fn estimate_sizes() {
        assert_ge!(
            super::estimate_from_size_and_name(42496, "Model/Animal_Bass.Tex1.sbfres", Endian::Big),
            Some(50688)
        );
        assert_ge!(
            super::estimate_from_size_and_name(
                7735632,
                "Model/DgnMrgPrt_Dungeon061.bfres",
                Endian::Little,
            ),
            Some(8658192)
        );
        assert_ge!(
            super::estimate_from_size_and_name(
                7735632,
                "Model/DgnMrgPrt_Dungeon061.bfres",
                Endian::Little,
            ),
            Some(8658192)
        );
        assert_ge!(
            super::estimate_from_size_and_name(
                1408,
                "Actor/GeneralParamList/Weapon_Bow_071.bgparamlist",
                Endian::Little,
            ),
            Some(8272)
        );
        assert_ge!(
            super::estimate_from_size_and_name(
                3540,
                "Actor/AIProgram/NpcGerudoQueenBattle.baiprog",
                Endian::Big,
            ),
            Some(9444)
        );
    }
}
