pub fn compliment_filter(c: f32, pair_values: (f32, f32)) -> f32 {
    (pair_values.0 * c) + (pair_values.1 * (1.0 - c))
}