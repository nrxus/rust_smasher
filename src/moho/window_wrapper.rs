extern crate glm;

pub fn wrap_rects(original: glm::IVec4, max: glm::UVec2) -> [Option<glm::IVec4>; 4] {
    let max = glm::ivec2(max.x as i32, max.y as i32);
    let left = (original.x + max.x) % max.x;
    let right = left + original.z;
    let top = (original.y + max.y) % max.y;
    let bottom = top + original.w;
    let original = glm::ivec4(left, top, original.z, original.w);

    let side = if left < 0 {
        Some(glm::ivec4(left + max.x, top, original.z, original.w))
    } else if right > max.x as i32 {
        Some(glm::ivec4((right % max.x) - original.z, top, original.z, original.w))
    } else {
        None
    };

    let vert = if top < 0 {
        Some(glm::ivec4(left, top + max.y, original.z, original.w))
    } else if bottom > max.y {
        Some(glm::ivec4(left, bottom % max.y - original.w, original.z, original.w))
    } else {
        None
    };

    let side_vert = match (side, vert) {
        (Some(side_center), Some(vert_center)) => {
            Some(glm::ivec4(side_center.x, vert_center.y, original.z, original.w))
        }
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
        let max_coords = glm::uvec2(40, 30);
        let rect = glm::ivec4(25, 15, 10, 7);
        let rects: Vec<_> = wrap_rects(rect, max_coords).iter().filter_map(|&x| x).collect();
        assert_eq!(rects.len(), 1);
        assert_eq!(rect, rects[0]);
    }

    #[test]
    fn wrapping_left() {
        let max_coords = glm::uvec2(40, 30);
        let rect = glm::ivec4(-2, 16, 10, 8);
        let rects: Vec<_> = wrap_rects(rect, max_coords).iter().filter_map(|&x| x).collect();
        assert_eq!(rects.len(), 2);
        assert_eq!(rect, rects[1]);
        assert_eq!(glm::ivec4(38, 16, 10, 8), rects[0]);
    }

    #[test]
    fn wrapping_right() {
        let max_coords = glm::uvec2(40, 30);
        let rect = glm::ivec4(34, 16, 10, 13);
        let rects: Vec<_> = wrap_rects(rect, max_coords).iter().filter_map(|&x| x).collect();
        assert_eq!(rects.len(), 2);
        assert_eq!(rect, rects[0]);
        assert_eq!(glm::ivec4(-6, 16, 10, 13), rects[1]);
    }

    #[test]
    fn wrapping_bottom() {
        let max_coords = glm::uvec2(40, 30);
        let rect = glm::ivec4(27, 21, 6, 10);
        let rects: Vec<_> = wrap_rects(rect, max_coords).iter().filter_map(|&x| x).collect();
        assert_eq!(rects.len(), 2);
        assert_eq!(rect, rects[0]);
        assert_eq!(glm::ivec4(27, -9, 6, 10), rects[1]);
    }

    #[test]
    fn wrapping_top() {
        let max_coords = glm::uvec2(40, 30);
        let rect = glm::ivec4(27, -3, 6, 10);
        let rects: Vec<_> = wrap_rects(rect, max_coords).iter().filter_map(|&x| x).collect();
        assert_eq!(rects.len(), 2);
        assert_eq!(rect, rects[1]);
        assert_eq!(glm::ivec4(27, 27, 6, 10), rects[0]);
    }

    #[test]
    fn wrapping_corner() {
        let max_coords = glm::uvec2(40, 30);
        let rect = glm::ivec4(-2, -3, 10, 10);
        let rects: Vec<_> = wrap_rects(rect, max_coords).iter().filter_map(|&x| x).collect();
        assert_eq!(rects.len(), 4);
        assert_eq!(rect, rects[3]);
        assert_eq!(glm::ivec4(38, -3, 10, 10), rects[2]);
        assert_eq!(glm::ivec4(-2, 27, 10, 10), rects[1]);
        assert_eq!(glm::ivec4(38, 27, 10, 10), rects[0]);
    }
}
