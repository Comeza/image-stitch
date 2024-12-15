pub trait ColorExt {
    fn add(&self, other: Self) -> Self;
    fn sub(&self, other: Self) -> Self;
    fn len(&self) -> f32;
}

impl ColorExt for [u8; 4] {
    fn len(&self) -> f32 {
        f32::sqrt(
            self.iter()
                .zip(self)
                .map(|(a, b)| (*a as f32 * *b as f32))
                .sum(),
        )
    }

    fn add(&self, other: Self) -> Self {
        [
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ]
    }

    fn sub(&self, other: Self) -> Self {
        [
            (self[0] as i16 - other[0] as i16).clamp(0, u8::MAX as i16) as u8,
            (self[1] as i16 - other[1] as i16).clamp(0, u8::MAX as i16) as u8,
            (self[2] as i16 - other[2] as i16).clamp(0, u8::MAX as i16) as u8,
            (self[3] as i16 - other[3] as i16).clamp(0, u8::MAX as i16) as u8,
        ]
    }
}
