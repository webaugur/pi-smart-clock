use super::FaceId;

/// SVG-space layout constants for mapping dial artwork to screen coordinates.
#[derive(Clone, Copy, Debug)]
pub struct FaceLayout {
    pub view_size: f32,
    pub center: f32,
    pub numeral_radius: f32,
}

impl FaceId {
    pub fn layout(self) -> FaceLayout {
        match self {
            FaceId::RetroRoman => FaceLayout {
                view_size: 512.0,
                center: 256.0,
                numeral_radius: 188.0,
            },
        }
    }
}

impl FaceLayout {
    pub fn screen_scale(&self, diameter: u32, raster_size: u32) -> f32 {
        diameter as f32 / raster_size as f32
    }

    pub fn numeral_anchor(&self, index: u32) -> (f32, f32, f32) {
        let angle = index as f32 * 30.0;
        let rad = angle.to_radians();
        let x = self.center + rad.sin() * self.numeral_radius;
        let y = self.center - rad.cos() * self.numeral_radius;
        (x, y, angle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xii_anchor_matches_svg_ring() {
        let face = FaceId::RetroRoman.layout();
        let (x, y, angle) = face.numeral_anchor(0);
        assert!((x - 256.0).abs() < 0.01);
        assert!((y - 68.0).abs() < 0.01);
        assert!((angle - 0.0).abs() < 0.01);
    }
}