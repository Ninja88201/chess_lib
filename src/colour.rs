use std::{fmt::Display, ops::Not};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Colour
{
    White,
    Black,
}
impl Colour {
    pub fn white(&self) -> bool {
        self == &Colour::White
    }
    pub fn black(&self) -> bool {
        self == &Colour::Black
    }
    pub fn new(b: bool) -> Self {
        match b {
            true => Colour::White,
            false => Colour::Black,
        }
    }
}
impl Display for Colour
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Colour::White => 'w',
            Colour::Black => 'b',
        };
        write!(f, "{}", c)?;
        Ok(())
    }
}
impl Not for Colour
{
    type Output = Colour;

    fn not(self) -> Self::Output {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}