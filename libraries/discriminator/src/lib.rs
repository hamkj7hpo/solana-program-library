use solana_program::program_error::ProgramError;

pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];

    fn discriminator() -> [u8; 8] {
        Self::DISCRIMINATOR
    }
}

#[macro_export]
macro_rules! discriminator {
    ($discriminator:expr) => {
        #[automatically_derived]
        impl Discriminator for $discriminator {
            const DISCRIMINATOR: [u8; 8] = [0; 8]; // Placeholder, replace with actual discriminator
        }
    };
}
