struct vec3 {
    x: float,
    y: float,
    z: float,
}

impl Eq for vec3 {
    #[inline(always)]
    fn eq(&self, other: &vec3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    #[inline(always)]
    fn ne(&self, other: &vec3) -> bool {
        !self.eq(other)
    }
}

impl Add<vec3,vec3> for vec3 {
    #[inline(always)]
    fn add(&self, other: &vec3) -> vec3 {
        vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub<vec3,vec3> for vec3 {
    #[inline(always)]
    fn sub(&self, other: &vec3) -> vec3 {
        vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul<float,vec3> for vec3 {
    #[inline(always)]
    fn mul(&self, n: &float) -> vec3 {
        vec3 { x: self.x * *n, y: self.y * *n, z: self.z * *n }
    }
}

impl Div<float,vec3> for vec3 {
    #[inline(always)]
    fn div(&self, n: &float) -> vec3 {
        let inv = 1.0 / *n;
        vec3 { x: self.x * inv, y: self.y * inv, z: self.z * inv }
    }
}

impl Neg<vec3> for vec3 {
    #[inline(always)]
    fn neg(&self) -> vec3 {
        vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

pub impl vec3 {
    #[inline(always)]
    fn cross(&self, other: &vec3) -> vec3 {
        vec3 { x: self.y*other.z - self.z*other.y, y: self.z*other.x - self.x*other.z, z: self.x*other.y - self.y*other.x }
    }

    #[inline(always)]
    fn dot(&self, other: &vec3) -> float {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    #[inline(always)]
    fn length_sqr(&self) -> float {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    #[inline(always)]
    fn length(&self) -> float {
        float::sqrt(self.length_sqr())
    }

    #[inline(always)]
    fn unit(&self) -> vec3 {
        let l = 1.0 / self.length();
        vec3 { x: self.x*l, y: self.y*l, z: self.z*l }
    }
}

#[inline(always)]
pub fn new(x: float, y: float, z: float) -> vec3 {
    vec3 { x: x, y: y, z: z }
}
