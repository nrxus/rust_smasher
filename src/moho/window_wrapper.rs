extern crate glm;

pub fn wrap_rects(original: glm::IVec4, max: glm::UVec2) -> [Option<glm::IVec4>; 4] {
    let center = glm::uvec2(((original.x + original.z / 2 + max.x as i32) % max.x as i32) as u32,
                            ((original.y + original.w / 2 + max.y as i32) % max.y as i32) as u32);

    let dims = glm::uvec2(original.z as u32, original.w as u32);
    let mut ret = [None, None, None, None];
    let centers = get_wrapped_centers(center, dims, max);
    for (i, c) in centers.iter().enumerate() {
        ret[i] = match c {
            &Some(center) => {
                let left = center.x - original.z / 2;
                let top = center.y - original.w / 2;
                let rect = glm::ivec4(left, top, original.z, original.w);
                Some(rect)
            }
            &None => None,
        }
    }
    ret
}

fn get_wrapped_centers(original: glm::UVec2,
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

    let centers = [Some(original), side, vert, side_vert];
    centers
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
        assert_eq!(rect, rects[0]);
        assert_eq!(glm::ivec4(38, 16, 10, 8), rects[1]);
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
        assert_eq!(rect, rects[0]);
        assert_eq!(glm::ivec4(27, 27, 6, 10), rects[1]);
    }

    #[test]
    fn wrapping_corner() {
        let max_coords = glm::uvec2(40, 30);
        let rect = glm::ivec4(-2, -3, 10, 10);
        let rects: Vec<_> = wrap_rects(rect, max_coords).iter().filter_map(|&x| x).collect();
        assert_eq!(rects.len(), 4);
        assert_eq!(rect, rects[0]);
        assert_eq!(glm::ivec4(38, -3, 10, 10), rects[1]);
        assert_eq!(glm::ivec4(-2, 27, 10, 10), rects[2]);
        assert_eq!(glm::ivec4(38, 27, 10, 10), rects[3]);
    }
}
