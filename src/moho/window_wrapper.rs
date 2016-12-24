extern crate glm;

pub fn get_wrapped_centers(original: glm::UVec2,
                           dims: glm::UVec2,
                           max_coords: glm::UVec2)
                           -> [Option<glm::IVec2>; 4] {
    let original = glm::ivec2(original.x as i32, original.y as i32);
    let half_dims = glm::ivec2(dims.x as i32 / 2, dims.y as i32 / 2);
    let left = original.x - half_dims.x;
    let right = original.x + half_dims.x;
    let bottom = original.y + half_dims.y;
    let top = original.y - half_dims.y;

    let side = if left < 0 {
        Some(glm::ivec2(left + max_coords.x as i32 + half_dims.x, original.y))
    } else if right > max_coords.x as i32 {
        Some(glm::ivec2(right % max_coords.x as i32 - half_dims.x, original.y))
    } else {
        None
    };

    let vert = if top < 0 {
        Some(glm::ivec2(original.x, top + max_coords.y as i32 + half_dims.y))
    } else if bottom > max_coords.y as i32 {
        Some(glm::ivec2(original.x, bottom % max_coords.y as i32 - half_dims.y))
    } else {
        None
    };

    let side_vert = match (side, vert) {
        (Some(side_center), Some(vert_center)) => Some(glm::ivec2(side_center.x, vert_center.y)),
        _ => None,
    };

    [Some(original), side, vert, side_vert]
}

#[cfg(test)]
mod test {
    extern crate glm;

    use super::*;

    #[test]
    fn no_wrapping() {
        let original = glm::uvec2(30, 20);
        let dims = glm::uvec2(10, 7);
        let max_coords = glm::uvec2(40, 30);
        let centers: Vec<_> =
            get_wrapped_centers(original, dims, max_coords).iter().filter_map(|&x| x).collect();

        let original = glm::ivec2(30, 20);
        assert_eq!(centers.len(), 1);
        assert_eq!(original, centers[0]);
    }

    #[test]
    fn wrapping_left() {
        let original = glm::uvec2(3, 20);
        let dims = glm::uvec2(10, 8);
        let max_coords = glm::uvec2(40, 30);
        let centers: Vec<_> =
            get_wrapped_centers(original, dims, max_coords).iter().filter_map(|&x| x).collect();

        let original = glm::ivec2(3, 20);
        let left = glm::ivec2(43, 20);
        assert_eq!(centers.len(), 2);
        assert_eq!(original, centers[0]);
        assert_eq!(left, centers[1]);
    }

    #[test]
    fn wrapping_right() {
        let original = glm::uvec2(39, 20);
        let dims = glm::uvec2(10, 13);
        let max_coords = glm::uvec2(40, 30);
        let centers: Vec<_> =
            get_wrapped_centers(original, dims, max_coords).iter().filter_map(|&x| x).collect();

        let original = glm::ivec2(39, 20);
        let right = glm::ivec2(-1, 20);
        assert_eq!(centers.len(), 2);
        assert_eq!(original, centers[0]);
        assert_eq!(right, centers[1]);
    }

    #[test]
    fn wrapping_bottom() {
        let original = glm::uvec2(30, 26);
        let dims = glm::uvec2(6, 10);
        let max_coords = glm::uvec2(40, 30);
        let centers: Vec<_> =
            get_wrapped_centers(original, dims, max_coords).iter().filter_map(|&x| x).collect();

        let original = glm::ivec2(30, 26);
        let bottom = glm::ivec2(30, -4);
        assert_eq!(centers.len(), 2);
        assert_eq!(original, centers[0]);
        assert_eq!(bottom, centers[1]);
    }

    #[test]
    fn wrapping_top() {
        let original = glm::uvec2(30, 2);
        let dims = glm::uvec2(6, 10);
        let max_coords = glm::uvec2(40, 30);
        let centers: Vec<_> =
            get_wrapped_centers(original, dims, max_coords).iter().filter_map(|&x| x).collect();

        let original = glm::ivec2(30, 2);
        let top = glm::ivec2(30, 32);
        assert_eq!(centers.len(), 2);
        assert_eq!(original, centers[0]);
        assert_eq!(top, centers[1]);
    }

    #[test]
    fn wrapping_corner() {
        let original = glm::uvec2(3, 2);
        let dims = glm::uvec2(10, 10);
        let max_coords = glm::uvec2(40, 30);
        let centers: Vec<_> =
            get_wrapped_centers(original, dims, max_coords).iter().filter_map(|&x| x).collect();

        let original = glm::ivec2(3, 2);
        let left = glm::ivec2(43, 2);
        let top = glm::ivec2(3, 32);
        let top_left = glm::ivec2(43, 32);
        assert_eq!(centers.len(), 4);
        assert_eq!(original, centers[0]);
        assert_eq!(left, centers[1]);
        assert_eq!(top, centers[2]);
        assert_eq!(top_left, centers[3]);
    }
}
