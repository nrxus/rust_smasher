use resource_manager::{Texture, TextureId};

use glm;

#[derive(Clone)]
pub struct TileSheet {
    id: TextureId,
    tiles: glm::UVec2,
    pub dimensions: glm::UVec2,
}

pub struct Tile {
    pub id: TextureId,
    pub src: glm::UVec4,
}

impl TileSheet {
    pub fn new(tiles: glm::UVec2, texture: Texture) -> TileSheet {
        let dimensions = texture.dims / tiles;
        TileSheet {
            id: texture.id,
            dimensions: dimensions,
            tiles: tiles,
        }
    }

    pub fn tile(&self, index: u32) -> Tile {
        let tile_pos = glm::uvec2(index % self.tiles.x, index / self.tiles.x);
        let position = tile_pos * self.dimensions;
        let src = glm::uvec4(position.x, position.y, self.dimensions.x, self.dimensions.y);

        Tile {
            id: self.id,
            src: src,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_frame() {
        let texture = Texture {
            id: 1,
            dims: glm::uvec2(10, 10),
        };
        let sheet = TileSheet::new(glm::uvec2(1, 1), texture);
        let tile = sheet.tile(0);
        assert_eq!(tile.id, 1);
        assert_eq!(tile.src, glm::uvec4(0, 0, 10, 10));
    }

    #[test]
    fn single_row() {
        let texture = Texture {
            id: 2,
            dims: glm::uvec2(10, 10),
        };
        let sheet = TileSheet::new(glm::uvec2(10, 1), texture);
        let tile = sheet.tile(4);
        assert_eq!(tile.id, 2);
        assert_eq!(tile.src, glm::uvec4(4, 0, 1, 10));
    }

    #[test]
    fn single_column() {
        let texture = Texture {
            id: 1,
            dims: glm::uvec2(10, 10),
        };
        let sheet = TileSheet::new(glm::uvec2(1, 5), texture);
        let tile = sheet.tile(4);
        assert_eq!(tile.id, 1);
        assert_eq!(tile.src, glm::uvec4(0, 8, 10, 2));
    }

    #[test]
    fn mult_frames() {
        let texture = Texture {
            id: 10,
            dims: glm::uvec2(20, 10),
        };
        let sheet = TileSheet::new(glm::uvec2(4, 2), texture);
        let tile = sheet.tile(5);
        assert_eq!(tile.id, 10);
        assert_eq!(tile.src, glm::uvec4(5, 5, 5, 5));
    }
}
