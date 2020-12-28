use std::ops::*;

//Vec3f is a wrapper for a Tuple of 3 f32s
#[derive(Debug, PartialEq, Clone)]
pub struct Vec3f(pub f32, pub f32, pub f32);

impl Vec3f {
    //Negates a Vec3f
    pub fn negate(&self) -> Vec3f {
        Vec3f(-self.0, -self.1, -self.2)
    }

    //Gets the magnitude of a Vec3f
    pub fn magnitude(vector: &Vec3f) -> f32 {
        ((vector.0 * vector.0) + (vector.1 * vector.1) + (vector.2 * vector.2)).sqrt()
    }

    //Normalizes a Vec3f
    pub fn normalize(&self) -> Vec3f {
        let magnitude = Vec3f::magnitude(&self);
        Vec3f(self.0 / magnitude, self.1 / magnitude, self.2 / magnitude)
    }

    //Finds the dot product of 2 Vec3f
    pub fn dot(vec1: &Vec3f, vec2: &Vec3f) -> f32 {
        (vec1.0 * vec2.0) + (vec1.1 * vec2.1) + (vec1.2 * vec2.2)
    }

    //Reflects a vector about a given normal
    pub fn reflect(vector: &Vec3f, normal: &Vec3f) -> Vec3f {
        vector - (normal * 2.0 * Vec3f::dot(vector, normal))
    }

    pub fn get(&self, index: usize) -> f32 {
        match index {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => panic!("Index is out of Vec2 range"),
        }
    }

    pub fn set(&mut self, index: usize, value: f32) {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            _ => panic!("Index is out of Vec2 range"),
        }
    }
}


//Vec3f + Vec3
impl Add for Vec3f {
    type Output = Vec3f;
    
    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
//&Vec3f + &Vec4
impl<'a, 'b> Add<&'b Vec3f> for &'a Vec3f {
    type Output = Vec3f;
    
    fn add(self, other: &'b Vec3f) -> Vec3f {
        Vec3f(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
//&Vec3f + Vec4
impl<'a> Add<Vec3f> for &'a Vec3f {
    type Output = Vec3f;
    
    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
//Vec3f + &Vec4
impl<'a> Add<&'a Vec3f> for Vec3f {
    type Output = Vec3f;
    
    fn add(self, other: &'a Vec3f) -> Vec3f {
        Vec3f(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

//Vec3f - Vec4
impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
//&Vec3f - &Vec4s
impl<'a, 'b> Sub<&'b Vec3f> for &'a Vec3f {
    type Output = Vec3f;
    
    fn sub(self, other: &'b Vec3f) -> Vec3f {
        Vec3f(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
//&Vec3f - Vec4
impl<'a> Sub<Vec3f> for &'a Vec3f {
    type Output = Vec3f;
    
    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
//Vec3f - &Vec4
impl<'a> Sub<&'a Vec3f> for Vec3f {
    type Output = Vec3f;
    
    fn sub(self, other: &'a Vec3f) -> Vec3f {
        Vec3f(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

//Vec3f * f32
impl Mul<f32> for Vec3f {
    type Output = Vec3f;
    
    fn mul(self, other: f32) -> Vec3f {
        Vec3f(self.0 * other, self.1 * other, self.2 * other)
    }
}
//&Vec3f * &f32
impl<'a, 'b> Mul<&'b f32> for &'a Vec3f {
    type Output = Vec3f;
    
    fn mul(self, other: &'b f32) -> Vec3f {
        Vec3f(self.0 * other, self.1 * other, self.2 * other)
    }
}
//&Vec3f * f32
impl<'a> Mul<f32> for &'a Vec3f {
    type Output = Vec3f;
    
    fn mul(self, other: f32) -> Vec3f {
        Vec3f(self.0 * other, self.1 * other, self.2 * other)
    }
}
//Vec3f * &f32
impl<'a> Mul<&'a f32> for Vec3f {
    type Output = Vec3f;
    
    fn mul(self, other: &'a f32) -> Vec3f {
        Vec3f(self.0 * other, self.1 * other, self.2 * other)
    }
}

//f32 * Vec3f
impl Mul<Vec3f> for f32 {
    type Output = Vec3f;
    
    fn mul(self, other: Vec3f) -> Vec3f {
        Vec3f(other.0 * self, other.1 * self, other.2 * self)
    }
}
//&f32 * &Vec3f
impl<'a, 'b> Mul<&'b Vec3f> for &'a f32 {
    type Output = Vec3f;
    
    fn mul(self, other: &'b Vec3f) -> Vec3f {
        Vec3f(other.0 * self, other.1 * self, other.2 * self)
    }
}
//&f32 * &Vec3f
impl<'a> Mul<Vec3f> for &'a f32 {
    type Output = Vec3f;
    
    fn mul(self, other: Vec3f) -> Vec3f {
        Vec3f(other.0 * self, other.1 * self, other.2 * self)
    }
}
//f32 * &Vec3f
impl<'a> Mul<&'a Vec3f> for f32 {
    type Output = Vec3f;
    
    fn mul(self, other: &'a Vec3f) -> Vec3f {
        Vec3f(other.0 * self, other.1 * self, other.2 * self)
    }
}

//Vec3f * Vec3
impl Mul for Vec3f {
    type Output = Vec3f;
    
    fn mul(self, other: Vec3f) -> Vec3f {
        Vec3f((self.1 * other.2) - (self.2 * other.1), (self.2 * other.0) - (self.0 * other.2), (self.0 * other.1) - (self.1 * other.0))
    }
}
//&Vec3f * &Vec3
impl<'a, 'b> Mul<&'b Vec3f> for &'a Vec3f {
    type Output = Vec3f;
    
    fn mul(self, other: &'b Vec3f) -> Vec3f {
        Vec3f((self.1 * other.2) - (self.2 * other.1), (self.2 * other.0) - (self.0 * other.2), (self.0 * other.1) - (self.1 * other.0))
    }
}
//&Vec3f * Vec3
impl<'a> Mul<Vec3f> for &'a Vec3f {
    type Output = Vec3f;
    
    fn mul(self, other: Vec3f) -> Vec3f {
        Vec3f((self.1 * other.2) - (self.2 * other.1), (self.2 * other.0) - (self.0 * other.2), (self.0 * other.1) - (self.1 * other.0))
    }
}
//Vec3f * &Vec3
impl<'a> Mul<&'a Vec3f> for Vec3f {
    type Output = Vec3f;
    
    fn mul(self, other: &'a Vec3f) -> Vec3f {
        Vec3f((self.1 * other.2) - (self.2 * other.1), (self.2 * other.0) - (self.0 * other.2), (self.0 * other.1) - (self.1 * other.0))
    }
}

//Vec2 is a wrapper for a Tuple of 2 f32s
#[derive(Debug, PartialEq, Clone)]
pub struct Vec2f(pub f32, pub f32);

impl Vec2f {
    pub fn get(&self, index: usize) -> f32 {
        match index {
            0 => self.0,
            1 => self.1,
            _ => panic!("Index is out of Vec2 range"),
        }
    }

    pub fn set(&mut self, index: usize, value: f32) {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            _ => panic!("Index is out of Vec2 range"),
        }
    }
}

//Vec2f + Vec2f
impl Add for Vec2f {
    type Output = Vec2f;
    
    fn add(self, other: Vec2f) -> Vec2f {
        Vec2f(self.0 + other.0, self.1 + other.1)
    }
}

//Vec2f * f32
impl Mul<f32> for Vec2f {
    type Output = Vec2f;
    
    fn mul(self, other: f32) -> Vec2f {
        Vec2f(self.0 * other, self.1 * other)
    }
}
//&Vec3f * &f32
impl<'a, 'b> Mul<&'b f32> for &'a Vec2f {
    type Output = Vec2f;
    
    fn mul(self, other: &'b f32) -> Vec2f {
        Vec2f(self.0 * other, self.1 * other)
    }
}
//&Vec3f * f32
impl<'a> Mul<f32> for &'a Vec2f {
    type Output = Vec2f;
    
    fn mul(self, other: f32) -> Vec2f {
        Vec2f(self.0 * other, self.1 * other)
    }
}
//Vec3f * &f32
impl<'a> Mul<&'a f32> for Vec2f {
    type Output = Vec2f;
    
    fn mul(self, other: &'a f32) -> Vec2f {
        Vec2f(self.0 * other, self.1 * other)
    }
}

//f32 * Vec3f
impl Mul<Vec2f> for f32 {
    type Output = Vec2f;
    
    fn mul(self, other: Vec2f) -> Vec2f {
        Vec2f(other.0 * self, other.1 * self)
    }
}
//&f32 * &Vec3f
impl<'a, 'b> Mul<&'b Vec2f> for &'a f32 {
    type Output = Vec2f;
    
    fn mul(self, other: &'b Vec2f) -> Vec2f {
        Vec2f(other.0 * self, other.1 * self)
    }
}
//&f32 * &Vec3f
impl<'a> Mul<Vec2f> for &'a f32 {
    type Output = Vec2f;
    
    fn mul(self, other: Vec2f) -> Vec2f {
        Vec2f(other.0 * self, other.1 * self)
    }
}
//f32 * &Vec3f
impl<'a> Mul<&'a Vec2f> for f32 {
    type Output = Vec2f;
    
    fn mul(self, other: &'a Vec2f) -> Vec2f {
        Vec2f(other.0 * self, other.1 * self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Vec2u(pub usize, pub usize);

impl Vec2u {
    pub fn get(&self, index: usize) -> usize {
        match index {
            0 => self.0,
            1 => self.1,
            _ => panic!("Index is out of Vec2 range"),
        }
    }

    pub fn set(&mut self, index: usize, value: usize) {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            _ => panic!("Index is out of Vec2 range"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Vec3u(pub usize, pub usize, pub usize);

impl Vec3u {
    pub fn get(&self, index: usize) -> usize {
        match index {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => panic!("Index is out of Vec3 range"),
        }
    }

    pub fn set(&mut self, index: usize, value: usize) {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            _ => panic!("Index is out of Vec3 range"),
        }
    }
}