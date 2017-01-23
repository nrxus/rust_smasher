use glm;

pub struct TileSheet {
    tiles: glm::UVec2,
}

impl TileSheet {
    pub fn new(tiles: glm::UVec2) -> TileSheet {
        TileSheet { tiles: tiles }
    }

    pub fn uv(&self, index: u32) -> glm::DVec4 {
        let x_tile = index % self.tiles.x;
        let y_tile = index / self.tiles.x;

        let tiles = glm::to_dvec2(self.tiles);
        let x_pos = x_tile as f64 / tiles.x;
        let y_pos = y_tile as f64 / tiles.y;
        let x_dims = 1. / tiles.x;
        let y_dims = 1. / tiles.y;

        glm::dvec4(x_pos, y_pos, x_dims, y_dims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_frame() {
        let sheet = TileSheet::new(glm::uvec2(1, 1));
        let expected = glm::dvec4(0., 0., 1., 1.);
        let actual = sheet.uv(0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn single_row() {
        let sheet = TileSheet::new(glm::uvec2(10, 1));
        let expected = glm::dvec4(0.4, 0., 0.1, 1.);
        let actual = sheet.uv(4);
        assert_eq!(actual, expected);
    }

    #[test]
    fn single_column() {
        let sheet = TileSheet::new(glm::uvec2(1, 5));
        let expected = glm::dvec4(0., 0.8, 1., 0.2);
        let actual = sheet.uv(4);
        assert_eq!(actual, expected);
    }

    #[test]
    fn mult_frames() {
        let sheet = TileSheet::new(glm::uvec2(4, 2));
        let expected = glm::dvec4(0.25, 0.5, 0.25, 0.5);
        let actual = sheet.uv(5);
        assert_eq!(actual, expected);
    }
}
