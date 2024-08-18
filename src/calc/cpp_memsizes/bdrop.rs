use std::mem::size_of;

use roead::aamp::ParameterIO;

use super::cpp_classes::DropTable::*;
use crate::Endian;

const CLASS_SIZE_WIIU: usize = std::mem::size_of::<Drop<u32>>();
const CLASS_SIZE_NX: usize = std::mem::size_of::<Drop<u64>>();

const OVERHEAD_WIIU: usize = 0x0;
const OVERHEAD_NX: usize = 0x0;
const HEADER_OVERHEAD_WIIU: usize = 0x20;
const HEADER_OVERHEAD_NX: usize = 0x18;

pub fn parse_size(bytes: &[u8], endian: Endian) -> Option<u32> {
    let mut total_size = match endian {
        Endian::Big => super::PARSE_CONST_WIIU + CLASS_SIZE_WIIU + OVERHEAD_WIIU,
        Endian::Little => super::PARSE_CONST_NX + CLASS_SIZE_NX + OVERHEAD_NX,
    };

    let a = ParameterIO::from_binary(bytes).ok()?;
    let (
        iter_size,
        header_size,
        table_size,
        item_size,
    );
    match endian {
        Endian::Big => {
            iter_size = super::ITER_CONST_WIIU;
            header_size = HEADER_OVERHEAD_WIIU;
            table_size = size_of::<Table<u32>>();
            item_size = size_of::<Item<u32>>();
        }
        Endian::Little => {
            // TODO: Why does this match ShopData's, but no others'?
            iter_size = super::ITER_CONST_NX + 8;
            header_size = HEADER_OVERHEAD_NX;
            table_size = size_of::<Table<u64>>();
            item_size = size_of::<Item<u64>>();
        }
    }

    if let Some(header) = a.param_root.objects.get("Header") {
        total_size += header_size;
        if let Some(num_tables_param) = header.get("TableNum") {
            let num_tables: usize = num_tables_param.as_int().ok()?;
            if num_tables > 0 {
                total_size += iter_size;
            }
            total_size += num_tables * table_size;
            for i in 0..num_tables {
                let table_id = format!("Table{:02}", i + 1);
                let table_name = header.get(table_id)?.as_string64().ok()?.as_str();
                if let Some(table) = a.param_root.objects.get(table_name) {
                    let num_items: usize = table.get("ColumnNum")?.as_int().ok()?;
                    if num_items > 0 {
                        total_size += iter_size;
                    }
                    total_size += num_items * item_size;
                }
            }
        }
    }
    Some(total_size as u32)
}
