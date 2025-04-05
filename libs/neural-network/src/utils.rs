#[macro_export]
/// This macro asserts that the difference between two
/// floating point numbers is less that EPSILON,
/// hence the name. I am aware that this is like
/// reinventing the wheel but I wanted to work with macros.
macro_rules! assert_almost_eq {
    ($left:expr, $right:expr) => {{
        let left: f32 = $left;
        let right: f32 = $right;

        assert!((left - right).abs() < f32::EPSILON);
    }};
}
