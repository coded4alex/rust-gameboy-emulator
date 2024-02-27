use crate::{app::utils::DataResult, memory::access::Memory};

struct TileData {
    pub tile_id: u8,
    pub tile_data: [u16; 8],
}

enum TileType {
    OBJ,
    BG,
    WIN,
}

pub fn get_tile(tile_id: u8, tile_type: TileType, mem: &Memory) -> DataResult<TileData> {
    let lcdc4: u8 = 0; // TODO: Has to be changed

    match tile_type {
        TileType::OBJ => get_obj(tile_id, mem),
        TileType::BG => get_bg_win(tile_id, mem, lcdc4),
        TileType::WIN => get_bg_win(tile_id, mem, lcdc4),
        _ => Err("Invalid tile_type".to_string()),
    }
}

fn get_obj(tile_id: u8, mem: &Memory) -> DataResult<TileData> {
    let base_index = 0x8000 + tile_id as u16 * 16;
    extract_tile_data(mem, base_index, tile_id)
}

fn get_bg_win(tile_id: u8, mem: &Memory, lcdc4: u8) -> DataResult<TileData> {
    let base_index = match lcdc4 {
        1 => {
            if tile_id < 128 {
                0x8000
            } else {
                0x8800
            }
        }
        0 => {
            if tile_id > 127 {
                0x8800
            } else {
                0x9000
            }
        }
        _ => return Err(format!("Invalid LCDC4 bit state {}", lcdc4)),
    };
    extract_tile_data(mem, base_index, tile_id)
}

fn extract_tile_data(mem: &Memory, base_index: u16, tile_id: u8) -> Result<TileData, String> {
    let mut tile_data = TileData {
        tile_id: tile_id,
        tile_data: [0; 8],
    };

    for i in 0..8 {
        tile_data.tile_data[i] = merge_bytes(mem.read(base_index + (i * 2) as u16), mem.read(base_index + (i * 2 + 1) as u16));
    }
    Ok(tile_data)
}

fn merge_bytes(first_byte: u8, second_byte: u8) -> u16 {
    let first_byte = first_byte as u16;
    let second_byte = second_byte as u16;

    let mut result: u16 = 0;
    for i in 0..8 {
        result = result | ((first_byte >> i & 1) | ((second_byte >> i & 1) << 1)) << (i * 2);
    }
    result
}

mod test {
    use crate::memory::{self, access::Memory};

    use super::{get_tile, merge_bytes};

    #[test]
    fn test_merge_bytes() {
        let result = merge_bytes(60, 126);
        assert_eq!(result, 12280)
    }

    #[test]
    fn test_tile_retrieval_obj() {
        let mut mem = Memory::create();
        mem.write(0x8000, 60);
        mem.write(0x8001, 126);
        mem.write(0x8800, 61);
        mem.write(0x8801, 126);

        let mut result_val = get_tile(0, super::TileType::OBJ, &mem).unwrap();
        assert_eq!(result_val.tile_data[0], 12280);

        result_val = get_tile(128, super::TileType::OBJ, &mem).unwrap();
        assert_eq!(result_val.tile_data[0], 12281);
    }

    #[test]
    fn test_file_retrievel_bg_win() {
        let mut mem = Memory::create();
        mem.write(0x8800, 60);
        mem.write(0x8801, 126);
        mem.write(0x9000, 61);
        mem.write(0x9001, 126);

        let mut result_val = get_tile(128, super::TileType::BG, &mem).unwrap();
        assert_eq!(result_val.tile_data[0], 12280);

        result_val = get_tile(0, super::TileType::BG, &mem).unwrap();
        assert_eq!(result_val.tile_data[0], 12281);
    }
}
