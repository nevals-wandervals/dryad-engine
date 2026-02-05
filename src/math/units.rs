use crate::impl_traits_for_units;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mass(pub f32);

impl Mass {
    pub fn new(mass: f32) -> Self {
        Self(mass)
    }
}

impl std::ops::Div<Volume> for Mass {
    type Output = Density;
    fn div(self, rhs: Volume) -> Self::Output {
        Density(self.0 / rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Volume(pub f32);

impl Volume {
    pub fn new(vol: f32) -> Self {
        Self(vol)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Density(pub f32);

impl Density {
    pub fn new(density: f32) -> Self {
        Self(density)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Temperature(pub f32);

impl Temperature {
    pub fn new(temp: f32) -> Self {
        Self(temp)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pressure(pub f32);

impl Pressure {
    pub fn new(press: f32) -> Self {
        Self(press)
    }
}

// -----
impl_traits_for_units!(Mass, Volume, Temperature, Pressure);
// -----

#[macro_export]
macro_rules! impl_traits_for_units {
    ($($id:ident),*) => {
        $(
            impl PartialEq<f32> for $id {
                fn eq(&self, other: &f32) -> bool {
                    self.0.eq(other)
                }
            }

            impl PartialOrd<f32> for $id {
                fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
                    self.0.partial_cmp(other)
                }
            }

            impl std::ops::Add<Self> for $id {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    Self::new(self.0 + rhs.0)
                }
            }

            impl std::ops::AddAssign<Self> for $id {
                fn add_assign(&mut self, rhs: Self) {
                    self.0 += rhs.0;
                }
            }

            impl std::ops::Add<f32> for $id {
                type Output = Self;
                fn add(self, rhs: f32) -> Self::Output {
                    Self::new(self.0.add(rhs))
                }
            }

            impl std::ops::AddAssign<f32> for $id {
                fn add_assign(&mut self, rhs: f32) {
                    self.0.add_assign(rhs);
                }
            }

            impl std::ops::Sub<Self> for $id {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    Self::new(self.0 - rhs.0)
                }
            }

            impl std::ops::SubAssign<Self> for $id {
                fn sub_assign(&mut self, rhs: Self) {
                    self.0 -= rhs.0;
                }
            }

            impl std::ops::Sub<f32> for $id {
                type Output = Self;
                fn sub(self, rhs: f32) -> Self::Output {
                    Self::new(self.0.sub(rhs))
                }
            }

            impl std::ops::SubAssign<f32> for $id {
                fn sub_assign(&mut self, rhs: f32) {
                    self.0.sub_assign(rhs);
                }
            }

            impl std::ops::Mul<Self> for $id {
                type Output = Self;
                fn mul(self, rhs: Self) -> Self::Output {
                    Self::new(self.0 * rhs.0)
                }
            }

            impl std::ops::MulAssign<Self> for $id {
                fn mul_assign(&mut self, rhs: Self) {
                    self.0 *= rhs.0;
                }
            }

            impl std::ops::Mul<f32> for $id {
                type Output = Self;
                fn mul(self, rhs: f32) -> Self::Output {
                    Self::new(self.0.mul(rhs))
                }
            }

            impl std::ops::MulAssign<f32> for $id {
                fn mul_assign(&mut self, rhs: f32) {
                    self.0.mul_assign(rhs);
                }
            }

            impl std::ops::Div<Self> for $id {
                type Output = Self;
                fn div(self, rhs: Self) -> Self::Output {
                    Self::new(self.0 / rhs.0)
                }
            }

            impl std::ops::DivAssign<Self> for $id {
                fn div_assign(&mut self, rhs: Self) {
                    self.0 /= rhs.0;
                }
            }

            impl std::ops::Div<f32> for $id {
                type Output = Self;
                fn div(self, rhs: f32) -> Self::Output {
                    Self::new(self.0.div(rhs))
                }
            }

            impl std::ops::DivAssign<f32> for $id {
                fn div_assign(&mut self, rhs: f32) {
                    self.0.div_assign(rhs);
                }
            }
        )*
    };
}
