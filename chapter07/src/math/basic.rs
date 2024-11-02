use std::f32::consts::PI;

pub fn to_radians(degree: f32) -> f32 {
    degree * PI / 180.0
}

pub fn to_degrees(radians: f32) -> f32 {
    radians * 180.0 / PI
}

pub fn near_zero(value: f32, epsilon: f32) -> bool {
    value.abs() <= epsilon
}

pub fn cot(angle: f32) -> f32 {
    1.0 / angle.tan()
}

pub fn lerp(a: f32, b: f32, f: f32) -> f32 {
    a + f * (b - a)
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::{to_degrees, to_radians};

    #[macro_export]
    macro_rules! assert_near_eq {
        ($left:expr, $right:expr, $epsilon:expr $(,)?) => {
            match (&$left, &$right, &$epsilon) {
                (left_val, right_val, epsilon_val) => {
                    use crate::math;
                    assert!(
                        math::basic::near_zero(*left_val - *right_val, *epsilon_val),
                        "`left == right` failed... left = {}, right = {}",
                        *left_val,
                        *right_val
                    );
                }
            }
        };
    }

    #[test]
    fn test_to_radians() {
        let expected = PI / 6.0;
        let actual = to_radians(30.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_to_degrees() {
        let expected = 30.0;
        let actual = to_degrees(PI / 6.0);
        assert_eq!(expected, actual);
    }
}
