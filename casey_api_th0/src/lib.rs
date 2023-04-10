// Investigate how to use prelude
// what is prelude

pub mod setup;
pub mod errors; 
pub mod game{
    pub mod admin;
    pub mod cards;
    pub mod players;
    mod thread {
        pub(crate) mod messages;
        pub(crate) mod core; 
    }
}

#[cfg(test)]
mod tests;