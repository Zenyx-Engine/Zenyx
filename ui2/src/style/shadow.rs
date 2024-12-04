use eframe::egui::{Color32, Vec2};

#[derive(Clone, Debug)]
pub struct Shadow {
    pub offset: Vec2,
    pub blur: f32,
    pub color: Color32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_creation() {
        let shadow = Shadow {
            offset: Vec2::new(0.0, 4.0),
            blur: 8.0,
            color: Color32::from_black_alpha(40),
        };
        assert_eq!(shadow.offset.x, 0.0);
        assert_eq!(shadow.offset.y, 4.0);
        assert_eq!(shadow.blur, 8.0);
    }
}