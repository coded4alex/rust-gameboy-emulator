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

impl TileData {
    pub fn get_tile(tile_id: u8, tile_type: TileType, mem: Memory) -> TileData {
        let lcdc4: u8 = 0; // TODO: Has to be changed

        match tile_type {
            TileType::OBJ => TileData { tile_id: 0, tile_data: [0; 8] },
            TileType::BG => TileData { tile_id: 0, tile_data: [0; 8] },
            TileType::WIN => TileData { tile_id: 0, tile_data: [0; 8] },
        }
    }
}

fn get_obj(tile_id: u8, mem: Memory) -> DataResult<TileData> {
    let mut tile_data = TileData {
        tile_id: tile_id,
        tile_data: [0; 8],
    };
    let base_index = (0x8000 + tile_id as u16 * 16) as u16;
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
    use super::merge_bytes;

    #[test]
    fn test_merge_bytes() {
        let result = merge_bytes(60, 126);
        assert_eq!(result, 12280)
    }
}
