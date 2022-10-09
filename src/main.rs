fn main() {
    println!("Hello, world!");
}

// video: https://youtu.be/kYB8IZa5AuE?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=364
#[derive(Copy, Clone, PartialEq, Debug)]
struct Matrix2D {
    v1: Vector2D, // [ v1.x, v2.x ]
    v2: Vector2D, // [ v1.y, v2.y ]
}

// video: https://youtu.be/XkY2DOUCWMU?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=119
impl std::ops::Mul for Matrix2D {
    type Output = Matrix2D;
    fn mul(self, rhs: Self) -> Self::Output {
        let left_m = self;
        let right_m = rhs;

        return Matrix2D {
            v1: right_m.v1.transform(left_m),
            v2: right_m.v2.transform(left_m),
        };
    }
}

// video: https://www.youtube.com/watch?v=fNk_zzaMoSs&list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&index=2
#[derive(PartialEq, Debug, Copy, Clone)]
struct Vector2D {
    x: f32,
    y: f32,
}

impl Vector2D {
    // video: https://youtu.be/fNk_zzaMoSs?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=414
    fn scale(&self, scalar: f32) -> Vector2D {
        return Vector2D {
            x: scalar * self.x,
            y: scalar * self.y,
        };
    }

    // video: https://youtu.be/kYB8IZa5AuE?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=210
    fn transform(&self, m: Matrix2D) -> Vector2D {
        return m.v1.scale(self.x) + m.v2.scale(self.y);
    }
}

// video: https://youtu.be/fNk_zzaMoSs?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=276
impl std::ops::Add for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Self) -> Self::Output {
        return Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl std::ops::Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: Self) -> Self::Output {
        return Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

// video: https://youtu.be/fNk_zzaMoSs?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=243
#[derive(PartialEq, Debug)]
struct Vector3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3D {
    // video: https://youtu.be/fNk_zzaMoSs?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=414
    fn scale(&self, scalar: f32) -> Vector3D {
        return Vector3D {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        };
    }
}

// video: https://youtu.be/fNk_zzaMoSs?list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&t=276
impl std::ops::Add for Vector3D {
    type Output = Vector3D;
    fn add(self, rhs: Self) -> Self::Output {
        return Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl std::ops::Sub for Vector3D {
    type Output = Vector3D;
    fn sub(self, rhs: Self) -> Self::Output {
        return Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    mod vector_2d {
        use crate::*;
        #[test]
        fn can_add_two_2d_vectors() {
            let v1 = Vector2D { x: 1.0, y: 2.0 };
            let v2 = Vector2D { x: 2.0, y: 3.0 };
            let result = v1 + v2;
            let expected = Vector2D { x: 3.0, y: 5.0 };
            assert_eq!(result, expected);
        }

        #[test]
        fn can_subtract_two_2d_vectors() {
            let v1 = Vector2D { x: 2.0, y: 3.0 };
            let v2 = Vector2D { x: 1.0, y: 1.0 };
            let result = v1 - v2;
            let expected = Vector2D { x: 1.0, y: 2.0 };
            assert_eq!(result, expected);
        }

        #[test]
        fn can_scale_a_vector_2d() {
            let v = Vector2D { x: 1.0, y: 2.0 };
            let scalar = 3.0;
            let expected = Vector2D { x: 3.0, y: 6.0 };
            let result = v.scale(scalar);
            assert_eq!(result, expected);
        }

        #[test]
        fn can_transform_a_vector_2d() {
            let v = Vector2D { x: 1.0, y: 1.0 };
            let rotate_180 = Matrix2D {
                v1: Vector2D { x: -1.0, y: 0.0 },
                v2: Vector2D { x: 0.0, y: -1.0 },
            };
            let expected = Vector2D { x: -1.0, y: -1.0 };
            let result = v.transform(rotate_180);
            assert_eq!(result, expected);
        }
    }

    mod vector_3d {
        use crate::*;

        #[test]
        fn can_add_two_3d_vectors() {
            let v1 = Vector3D {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let v2 = Vector3D {
                x: 2.0,
                y: 3.0,
                z: 6.0,
            };
            let result = v1 + v2;
            let expected = Vector3D {
                x: 3.0,
                y: 5.0,
                z: 9.0,
            };
            assert_eq!(result, expected);
        }

        #[test]
        fn can_subtract_two_3d_vectors() {
            let v1 = Vector3D {
                x: 2.0,
                y: 5.0,
                z: 7.0,
            };
            let v2 = Vector3D {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let result = v1 - v2;
            let expected = Vector3D {
                x: 1.0,
                y: 3.0,
                z: 4.0,
            };
            assert_eq!(result, expected);
        }

        #[test]
        fn can_scale_a_vector_3d() {
            let v = Vector3D {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let scalar = 3.0;
            let expected = Vector3D {
                x: 3.0,
                y: 6.0,
                z: 9.0,
            };
            let result = v.scale(scalar);
            assert_eq!(result, expected);
        }
    }

    mod matrix_2d {
        use crate::*;

        #[test]
        fn can_multiply_two_2d_matrices() {
            let rotate_90 = Matrix2D {
                v1: Vector2D { x: 0.0, y: -1.0 },
                v2: Vector2D { x: 1.0, y: 0.0 },
            };
            let scale_x_by_2 = Matrix2D {
                v1: Vector2D { x: 2.0, y: 0.0 },
                v2: Vector2D { x: 0.0, y: 1.0 },
            };
            // expected result of rotating then scaling
            let expected = Matrix2D {
                v1: Vector2D { x: 0.0, y: -1.0 },
                v2: Vector2D { x: 2.0, y: 0.0 },
            };
            let result = scale_x_by_2 * rotate_90;
            assert_eq!(result, expected);
        }
    }
}
