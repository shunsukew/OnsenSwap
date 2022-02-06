use brush::contracts::traits::psp22::{
    extensions::metadata::*,
    *,
};

#[brush::wrapper]
pub type PSP22TokenRef = dyn PSP22 + PSP22Metadata;

#[brush::trait_definition]
pub trait PSP22Token: PSP22 + PSP22Metadata {}
