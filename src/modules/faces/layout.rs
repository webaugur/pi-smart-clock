use super::FaceId;

/// One SVG clock hand asset (separate file per hand under `assets/faces/<face>/`).
#[derive(Clone, Copy, Debug)]
pub struct HandAsset {
    /// Path relative to `assets/faces/` (e.g. `retro-roman/hour-hand.svg`).
    pub file: &'static str,
    /// Distance in SVG units from pivot (viewBox centre) to tip at 12 o'clock.
    pub design_length: f32,
}

/// SVG-space layout constants for mapping dial artwork to screen coordinates.
#[derive(Clone, Copy, Debug)]
pub struct FaceLayout {
    pub view_size: f32,
    pub center: f32,
    pub numeral_radius: f32,
    pub hour_hand: HandAsset,
    pub minute_hand: HandAsset,
    pub second_hand: HandAsset,
    pub hub: Option<HandAsset>,
    /// On-screen diameter for the centre hub sprite (pixels).
    pub hub_screen_diameter: i32,
}

impl FaceId {
    pub fn layout(self) -> FaceLayout {
        match self {
            FaceId::RetroRoman => FaceLayout {
                view_size: 512.0,
                center: 256.0,
                numeral_radius: 188.0,
                hour_hand: HandAsset {
                    file: "retro-roman/hour-hand.svg",
                    design_length: 125.0,
                },
                minute_hand: HandAsset {
                    file: "retro-roman/minute-hand.svg",
                    design_length: 177.0,
                },
                second_hand: HandAsset {
                    file: "retro-roman/second-hand.svg",
                    design_length: 216.0,
                },
                hub: Some(HandAsset {
                    file: "retro-roman/hub.svg",
                    design_length: 20.0,
                }),
                hub_screen_diameter: 22,
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