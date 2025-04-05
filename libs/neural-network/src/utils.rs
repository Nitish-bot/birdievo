#[macro_export]
macro_rules! assert_almost_eq {
    ($left:expr, $right:expr) => {
        let left: f32 = $left;
        let right: f32 = $right;

        assert!((left - right).abs() < f32::EPSILON);
    }
}
