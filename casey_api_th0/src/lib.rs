// Investigate how to use prelude
// what is prelude

pub mod setup;
pub mod errors; 
pub mod game{
    pub mod admin;
    pub mod cards;
    pub mod players;
}

#[cfg(test)]
mod tests;