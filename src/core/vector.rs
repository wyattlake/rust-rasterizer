use std::ops::*;

//Vec3 is a wrapper for a Tuple of 3 f32s
#[derive(Debug, PartialEq, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    //Negates a Vec3
    pub fn negate(&self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }

    //Gets the magnitude of a Vec3
    pub fn magnitude(vector: &Vec3) -> f32 {
        ((vector.0 * vector.0) + (vector.1 * vector.1) + (vector.2 * vector.2)).sqrt()
    }

    //Normalizes a Vec3
    pub fn normalize(&self) -> Vec3 {
        let magnitude = Vec3::magnitude(&self);
        Vec3(self.0 / magnitude, self.1 / magnitude, self.2 / magnitude)
    }

    //Finds the dot product of 2 Vec3
    pub fn dot(vec1: &Vec3, vec2: &Vec3) -> f32 {
        (vec1.0 * vec2.0) + (vec1.1 * vec2.1) + (vec1.2 * vec2.2)
    }

    //Reflects a vector about a given normal
    pub fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
        vector - (normal * 2.0 * Vec3::dot(vector, normal))
    }

    pub fn get(&self, index: i32) -> f32 {
        match index {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => panic!("Index is out of Vec2 range"),
        }
    }

    pub fn set(&mut self, index: i32, value: f32) {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            _ => panic!("Index is out of Vec2 range"),
        }
    }
}


//Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;
    
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
//&Vec3 + &Vec4
impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    
    fn add(self, other: &'b Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
//&Vec3 + Vec4
impl<'a> Add<Vec3> for &'a Vec3 {
    type Output = Vec3;
    
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
//Vec3 + &Vec4
impl<'a> Add<&'a Vec3> for Vec3 {
    type Output = Vec3;
    
    fn add(self, other: &'a Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

//Vec3 - Vec4
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
//&Vec3 - &Vec4s
impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    
    fn sub(self, other: &'b Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
//&Vec3 - Vec4
impl<'a> Sub<Vec3> for &'a Vec3 {
    type Output = Vec3;
    
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
//Vec3 - &Vec4
impl<'a> Sub<&'a Vec3> for Vec3 {
    type Output = Vec3;
    
    fn sub(self, other: &'a Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

//Vec3 * f32
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    
    fn mul(self, other: f32) -> Vec3 {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}
//&Vec3 * &f32
impl<'a, 'b> Mul<&'b f32> for &'a Vec3 {
    type Output = Vec3;
    
    fn mul(self, other: &'b f32) -> Vec3 {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}
//&Vec3 * f32
impl<'a> Mul<f32> for &'a Vec3 {
    type Output = Vec3;
    
    fn mul(self, other: f32) -> Vec3 {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}
//Vec3 * &f32
impl<'a> Mul<&'a f32> for Vec3 {
    type Output = Vec3;
    
    fn mul(self, other: &'a f32) -> Vec3 {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

//f32 * Vec3
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(other.0 * self, other.1 * self, other.2 * self)
    }
}
//&f32 * &Vec3
impl<'a, 'b> Mul<&'b Vec3> for &'a f32 {
    type Output = Vec3;
    
    fn mul(self, other: &'b Vec3) -> Vec3 {
        Vec3(other.0 * self, other.1 * self, other.2 * self)
    }
}
//&f32 * &Vec3
impl<'a> Mul<Vec3> for &'a f32 {
    type Output = Vec3;
    
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(other.0 * self, other.1 * self, other.2 * self)
    }
}
//f32 * &Vec3
impl<'a> Mul<&'a Vec3> for f32 {
    type Output = Vec3;
    
    fn mul(self, other: &'a Vec3) -> Vec3 {
        Vec3(other.0 * self, other.1 * self, other.2 * self)
    }
}

//Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;
    
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3((self.1 * other.2) - (self.2 * other.1), (self.2 * other.0) - (self.0 * other.2), (self.0 * other.1) - (self.1 * other.0))
    }
}
//&Vec3 * &Vec3
impl<'a, 'b> Mul<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    
    fn mul(self, other: &'b Vec3) -> Vec3 {
        Vec3((self.1 * other.2) - (self.2 * other.1), (self.2 * other.0) - (self.0 * other.2), (self.0 * other.1) - (self.1 * other.0))
    }
}
//&Vec3 * Vec3
impl<'a> Mul<Vec3> for &'a Vec3 {
    type Output = Vec3;
    
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3((self.1 * other.2) - (self.2 * other.1), (self.2 * other.0) - (self.0 * other.2), (self.0 * other.1) - (self.1 * other.0))
    }
}
//Vec3 * &Vec3
impl<'a> Mul<&'a Vec3> for Vec3 {
    type Output = Vec3;
    
    fn mul(self, other: &'a Vec3) -> Vec3 {
        Vec3((self.1 * other.2) - (self.2 * other.1), (self.2 * other.0) - (self.0 * other.2), (self.0 * other.1) - (self.1 * other.0))
    }
}

//Vec2 is a wrapper for a Tuple of 2 f32s
#[derive(Debug, PartialEq, Clone)]
pub struct Vec2(pub f32, pub f32);

impl Vec2 {
    pub fn get(&self, index: i32) -> f32 {
        match index {
            0 => self.0,
            1 => self.1,
            _ => panic!("Index is out of Vec2 range"),
        }
    }

    pub fn set(&mut self, index: i32, value: f32) {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            _ => panic!("Index is out of Vec2 range"),
        }
    }
}