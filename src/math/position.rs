#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn get_mut_x(&mut self) -> &mut i32 {
        &mut self.x
    }

    pub fn set_x(&mut self, value: i32) {
        self.x = value;
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn get_mut_y(&mut self) -> &mut i32 {
        &mut self.y
    }

    pub fn set_y(&mut self, value: i32) {
        self.y = value;
    }

    pub fn to_index(self, row_width: i32) -> usize {
        super::get_index(self, row_width)
    }
}

impl std::ops::Add<i32> for Position {
    type Output = Self;
    fn add(mut self, rhs: i32) -> Self::Output {
        self.x += rhs;
        self.y += rhs;

        self
    }
}

impl std::ops::Add<(i32, i32)> for Position {
    type Output = Self;
    fn add(mut self, rhs: (i32, i32)) -> Self::Output {
        self.x += rhs.0;
        self.y += rhs.1;

        self
    }
}

impl std::ops::AddAssign<i32> for Position {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl std::ops::AddAssign<(i32, i32)> for Position {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl std::ops::Sub<i32> for Position {
    type Output = Self;
    fn sub(mut self, rhs: i32) -> Self::Output {
        self.x -= rhs;
        self.y -= rhs;

        self
    }
}

impl std::ops::Sub<(i32, i32)> for Position {
    type Output = Self;
    fn sub(mut self, rhs: (i32, i32)) -> Self::Output {
        self.x -= rhs.0;
        self.y -= rhs.1;

        self
    }
}

impl std::ops::SubAssign<i32> for Position {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl std::ops::SubAssign<(i32, i32)> for Position {
    fn sub_assign(&mut self, rhs: (i32, i32)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
    }
}

impl std::ops::Mul<i32> for Position {
    type Output = Self;
    fn mul(mut self, rhs: i32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;

        self
    }
}

impl std::ops::Mul<(i32, i32)> for Position {
    type Output = Self;
    fn mul(mut self, rhs: (i32, i32)) -> Self::Output {
        self.x *= rhs.0;
        self.y *= rhs.1;

        self
    }
}

impl std::ops::MulAssign<i32> for Position {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::MulAssign<(i32, i32)> for Position {
    fn mul_assign(&mut self, rhs: (i32, i32)) {
        self.x *= rhs.0;
        self.y *= rhs.1;
    }
}

impl std::ops::Div<i32> for Position {
    type Output = Self;
    fn div(mut self, rhs: i32) -> Self::Output {
        self.x /= rhs;
        self.y /= rhs;

        self
    }
}

impl std::ops::Div<(i32, i32)> for Position {
    type Output = Self;
    fn div(mut self, rhs: (i32, i32)) -> Self::Output {
        self.x /= rhs.0;
        self.y /= rhs.1;

        self
    }
}

impl std::ops::DivAssign<i32> for Position {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl std::ops::DivAssign<(i32, i32)> for Position {
    fn div_assign(&mut self, rhs: (i32, i32)) {
        self.x /= rhs.0;
        self.y /= rhs.1;
    }
}

#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        Position::new($x, $y)
    };
}
