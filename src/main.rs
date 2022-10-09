fn main() {
    println!("Hello, world!");
}

// video: https://www.youtube.com/watch?v=fNk_zzaMoSs&list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&index=2
#[derive(PartialEq, Debug)]
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
