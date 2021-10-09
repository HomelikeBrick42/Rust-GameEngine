pub type Vector3f = Vector3<f32>;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

macro_rules! vec3_op {
    ($n:ident, $f:ident, $op:tt) => {
        impl<T: std::ops::$n<Output = T>> std::ops::$n<Vector3<T>> for Vector3<T> {
            type Output = Self;
            fn $f(self, rhs: Vector3<T>) -> Self {
                Self {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                    z: self.z $op rhs.z,
                }
            }
        }

        impl<T: std::ops::$n<Output = T> + Copy> std::ops::$n<T> for Vector3<T> {
            type Output = Self;
            fn $f(self, rhs: T) -> Self {
                Self {
                    x: self.x $op rhs,
                    y: self.y $op rhs,
                    z: self.z $op rhs,
                }
            }
        }
    };
}

vec3_op!(Add, add, +);
vec3_op!(Sub, sub, -);
vec3_op!(Mul, mul, *);
vec3_op!(Div, div, /);
