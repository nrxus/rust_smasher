use glm;

pub fn rect_from_center(center: glm::IVec2, dimensions: glm::UVec2) -> glm::IVec4 {
    let dimensions = glm::to_ivec2(dimensions);
    (center - dimensions / 2).extend(dimensions.x).extend(dimensions.y)
}
