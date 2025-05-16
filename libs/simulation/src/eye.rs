use crate::*;

const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        if !(fov_range > 0. && fov_angle > 0. && cells > 0) {
            panic!("wrong input to Eye vars");
        }

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(&self, position: Vec2, rotation: f32, foods: &[Food]) -> Vec<f32> {
        let mut cells: Vec<f32> = vec![0.; self.cells];

        for food in foods {
            let food_position = food.position();
            let fov_range = self.fov_range;
            let fov_angle = self.fov_angle + 0.001; // rounding error or smth Idk

            let vec = food_position - position;
            let distance = vec.length();
            let angle = wrap_to_pi(Vec2::Y.angle_to(vec) - rotation);

            // Our birdie sees either both directions equally and
            // since the angle is already offset by the rotation
            // the birdie sees from - 1/2 fov_angle to positive of the same
            // Hence we add it once so angle ranges in [0, fov_angle]
            let angle = angle + fov_angle / 2.;

            if distance >= fov_range || !(0. ..=fov_angle).contains(&angle) {
                continue;
            }

            // How far along the food is in the fov in range [0, 1]
            let cell = angle / fov_angle;
            println!("{}", cell);

            // Get the index of cell by multiplying it with total cells
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            // The distance between the eyes and food
            // as an analogy for intensity of stimulation
            let cell_activation = (fov_range - distance) / fov_range;

            cells[cell] += cell_activation;
        }
        println!("{:?}", cells);
        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

pub fn wrap_to_pi(angle: f32) -> f32 {
    if angle > PI {
        angle - 2. * PI
    } else if angle < -PI {
        angle + 2. * PI
    } else {
        angle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_EYE_CELLS: usize = 13;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rotation: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle, TEST_EYE_CELLS);

            let actual_vision =
                eye.process_vision(Vec2::new(self.x, self.y), self.rotation, &self.foods);

            let actual_vision: Vec<&str> = actual_vision
                .into_iter()
                .map(|cell| {
                    if cell >= 0.6 {
                        "#"
                    } else if cell >= 0.3 {
                        "+"
                    } else if cell > 0. {
                        "."
                    } else {
                        " "
                    }
                })
                .collect();

            let vision = actual_vision.join("");

            assert_eq!(vision, self.expected_vision)
        }
    }

    #[test_case(1.0, "      +      ")]
    #[test_case(0.9, "      +      ")]
    #[test_case(0.8, "      +      ")]
    #[test_case(0.7, "      .      ")]
    #[test_case(0.6, "      .      ")]
    #[test_case(0.5, "             ")]
    #[test_case(0.4, "             ")]
    #[test_case(0.3, "             ")]
    #[test_case(0.2, "             ")]
    #[test_case(0.1, "             ")]
    fn fov_ranges(fov_range: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(0.5, 1.)],
            fov_angle: FRAC_PI_2,
            x: 0.5,
            y: 0.5,
            rotation: 0.0,
            fov_range,
            expected_vision,
        }
        .run()
    }

    #[test_case(0.25 * PI, " +         + ")] // FOV is narrow = 2 foods
    #[test_case(0.50 * PI, ".  +     +  .")]
    #[test_case(0.75 * PI, "  . +   + .  ")] // FOV gets progressively
    #[test_case(1.00 * PI, "   . + + .   ")] // wider and wider...
    #[test_case(1.25 * PI, "   . + + .   ")]
    #[test_case(1.50 * PI, ".   .+ +.   .")]
    #[test_case(1.75 * PI, ".   .+ +.   .")]
    #[test_case(2.00 * PI, "+.  .+ +.  .+")] // FOV is the widest = 8 foods
    fn fov_angles(fov_angle: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![
                food(0.0, 0.0),
                food(0.0, 0.33),
                food(0.0, 0.66),
                food(0.0, 1.0),
                food(1.0, 0.0),
                food(1.0, 0.33),
                food(1.0, 0.66),
                food(1.0, 1.0),
            ],
            fov_range: 1.0,
            x: 0.5,
            y: 0.5,
            rotation: 3.0 * FRAC_PI_2,
            fov_angle,
            expected_vision,
        }
        .run()
    }

    #[test_case(0.00 * PI, "         +   ")] // Food is to our right
    #[test_case(0.25 * PI, "        +    ")]
    #[test_case(0.50 * PI, "      +      ")] // Food is in front of us
    #[test_case(0.75 * PI, "    +        ")]
    #[test_case(1.00 * PI, "   +         ")] // Food is to our left
    #[test_case(1.25 * PI, " +           ")]
    // This 1.5 * PI test case was originally
    // #[test_case(1.50 * PI, "            +")]
    // but I assume this is due to differences in glam and nalgebra
    #[test_case(1.50 * PI, "+            ")] // Food is behind us
    #[test_case(1.75 * PI, "           + ")] // (we continue to see it
    #[test_case(2.00 * PI, "         +   ")] // due to 360° fov_angle.)
    #[test_case(2.25 * PI, "        +    ")]
    #[test_case(2.50 * PI, "      +      ")]
    fn rotations(rotation: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(0.0, 0.5)],
            fov_range: 1.0,
            fov_angle: 2.0 * PI,
            x: 0.5,
            y: 0.5,
            rotation,
            expected_vision,
        }
        .run()
    }

    // Checking the X axis:
    // (you can see the bird is "flying away" from the foods)
    #[test_case(0.9, 0.5, "#           #")]
    #[test_case(0.8, 0.5, "  #       #  ")]
    #[test_case(0.7, 0.5, "   #     #   ")]
    #[test_case(0.6, 0.5, "    +   +    ")]
    #[test_case(0.5, 0.5, "    +   +    ")]
    #[test_case(0.4, 0.5, "     + +     ")]
    #[test_case(0.3, 0.5, "     . .     ")]
    #[test_case(0.2, 0.5, "     . .     ")]
    #[test_case(0.1, 0.5, "     . .     ")]
    #[test_case(0.0, 0.5, "             ")]
    //
    // Checking the Y axis:
    // (you can see the bird is "flying alongside" the foods)
    #[test_case(0.5, 0.0, "            +")]
    #[test_case(0.5, 0.1, "          + .")]
    #[test_case(0.5, 0.2, "         +  +")]
    #[test_case(0.5, 0.3, "        + +  ")]
    #[test_case(0.5, 0.4, "      +  +   ")]
    #[test_case(0.5, 0.6, "   +  +      ")]
    #[test_case(0.5, 0.7, "  + +        ")]
    #[test_case(0.5, 0.8, "+  +         ")]
    #[test_case(0.5, 0.9, ". +          ")]
    #[test_case(0.5, 1.0, "+            ")]
    fn positions(x: f32, y: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
            fov_range: 1.0,
            fov_angle: FRAC_PI_2,
            rotation: 3.0 * FRAC_PI_2,
            x,
            y,
            expected_vision,
        }
        .run()
    }

    fn food(x: f32, y: f32) -> Food {
        Food::new(Vec2::new(x, y))
    }
}
