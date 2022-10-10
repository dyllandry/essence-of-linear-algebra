fn main() {
    println!("Hello, world!");
}

// video: https://www.youtube.com/watch?v=rHLEWRxRGiM&list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&index=6
#[derive(Copy, Clone, PartialEq, Debug)]
struct Matrix3 {
    col1: Vector3,
    col2: Vector3,
    col3: Vector3,
}

// video: https://youtu.be/kYB8IZa5AuE?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=364
#[derive(Copy, Clone, PartialEq, Debug)]
struct Matrix2 {
    col1: Vector2,
    col2: Vector2,
}

// video: https://youtu.be/XkY2DOUCWMU?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=119
impl std::ops::Mul for Matrix2 {
    type Output = Matrix2;
    fn mul(self, rhs: Self) -> Self::Output {
        let left_m = self;
        let right_m = rhs;

        return Matrix2 {
            col1: right_m.col1.transform(left_m),
            col2: right_m.col2.transform(left_m),
        };
    }
}

// video: https://www.youtube.com/watch?v=fNk_zzaMoSs&list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&index=2
#[derive(PartialEq, Debug, Copy, Clone)]
struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    // video: https://youtu.be/fNk_zzaMoSs?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=414
    fn scale(&self, scalar: f32) -> Vector2 {
        return Vector2 {
            x: scalar * self.x,
            y: scalar * self.y,
        };
    }

    // video: https://youtu.be/kYB8IZa5AuE?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=210
    fn transform(&self, m: Matrix2) -> Vector2 {
        return m.col1.scale(self.x) + m.col2.scale(self.y);
    }
}

// video: https://youtu.be/fNk_zzaMoSs?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=276
impl std::ops::Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        return Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Self) -> Self::Output {
        return Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

// video: https://youtu.be/fNk_zzaMoSs?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=243
#[derive(Copy, Clone, PartialEq, Debug)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    // video: https://youtu.be/fNk_zzaMoSs?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=414
    fn scale(&self, scalar: f32) -> Vector3 {
        return Vector3 {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        };
    }

    // video: https://www.youtube.com/watch?v=rHLEWRxRGiM&list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&index=6
    fn transform(&self, m: Matrix3) -> Vector3 {
        let scaled_col1 = m.col1.scale(self.x);
        let scaled_col2 = m.col2.scale(self.y);
        let scaled_col3 = m.col3.scale(self.z);
        return Vector3 {
            x: scaled_col1.x + scaled_col2.x + scaled_col3.x,
            y: scaled_col1.y + scaled_col2.y + scaled_col3.y,
            z: scaled_col1.z + scaled_col2.z + scaled_col3.z,
        };
    }
}

// video: https://youtu.be/fNk_zzaMoSs?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=276
impl std::ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Self) -> Self::Output {
        return Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Self) -> Self::Output {
        return Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    mod vector2 {
        use crate::*;

        #[test]
        fn can_add_two_vector2() {
            let v1 = Vector2 { x: 1.0, y: 2.0 };
            let v2 = Vector2 { x: 2.0, y: 3.0 };
            let result = v1 + v2;
            let expected = Vector2 { x: 3.0, y: 5.0 };
            assert_eq!(result, expected);
        }

        #[test]
        fn can_subtract_two_vector2() {
            let v1 = Vector2 { x: 2.0, y: 3.0 };
            let v2 = Vector2 { x: 1.0, y: 1.0 };
            let result = v1 - v2;
            let expected = Vector2 { x: 1.0, y: 2.0 };
            assert_eq!(result, expected);
        }

        #[test]
        fn can_scale_a_vector2() {
            let v = Vector2 { x: 1.0, y: 2.0 };
            let scalar = 3.0;
            let expected = Vector2 { x: 3.0, y: 6.0 };
            let result = v.scale(scalar);
            assert_eq!(result, expected);
        }

        #[test]
        fn can_transform_a_vector2() {
            let v = Vector2 { x: 1.0, y: 1.0 };
            let rotate_180 = Matrix2 {
                col1: Vector2 { x: -1.0, y: 0.0 },
                col2: Vector2 { x: 0.0, y: -1.0 },
            };
            let expected = Vector2 { x: -1.0, y: -1.0 };
            let result = v.transform(rotate_180);
            assert_eq!(result, expected);
        }
    }

    mod vector3 {
        use crate::*;

        #[test]
        fn can_add_two_vector3() {
            let v1 = Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let v2 = Vector3 {
                x: 2.0,
                y: 3.0,
                z: 6.0,
            };
            let result = v1 + v2;
            let expected = Vector3 {
                x: 3.0,
                y: 5.0,
                z: 9.0,
            };
            assert_eq!(result, expected);
        }

        #[test]
        fn can_subtract_two_vector3() {
            let v1 = Vector3 {
                x: 2.0,
                y: 5.0,
                z: 7.0,
            };
            let v2 = Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let result = v1 - v2;
            let expected = Vector3 {
                x: 1.0,
                y: 3.0,
                z: 4.0,
            };
            assert_eq!(result, expected);
        }

        #[test]
        fn can_scale_a_vector3() {
            let v = Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let scalar = 3.0;
            let expected = Vector3 {
                x: 3.0,
                y: 6.0,
                z: 9.0,
            };
            let result = v.scale(scalar);
            assert_eq!(result, expected);
        }

        #[test]
        fn can_transform_a_vector3() {
            let v = Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let m = Matrix3 {
                col1: Vector3 {
                    x: 1.0,
                    y: 4.0,
                    z: 7.0,
                },
                col2: Vector3 {
                    x: 2.0,
                    y: 5.0,
                    z: 8.0,
                },
                col3: Vector3 {
                    x: 3.0,
                    y: 6.0,
                    z: 9.0,
                },
            };
            let expected = Vector3 {
                x: 14.0,
                y: 32.0,
                z: 50.0,
            };
            let result = v.transform(m);
            assert_eq!(result, expected);
        }
    }

    mod matrix2 {
        use crate::*;

        #[test]
        fn can_multiply_two_matrix2() {
            let rotate_90 = Matrix2 {
                col1: Vector2 { x: 0.0, y: -1.0 },
                col2: Vector2 { x: 1.0, y: 0.0 },
            };
            let scale_x_by_2 = Matrix2 {
                col1: Vector2 { x: 2.0, y: 0.0 },
                col2: Vector2 { x: 0.0, y: 1.0 },
            };
            // expected result of rotating then scaling
            let expected = Matrix2 {
                col1: Vector2 { x: 0.0, y: -1.0 },
                col2: Vector2 { x: 2.0, y: 0.0 },
            };
            let result = scale_x_by_2 * rotate_90;
            assert_eq!(result, expected);
        }
    }
}
