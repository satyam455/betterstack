use config::Config;
use diesel::prelude::*;
use std::{
    env::{self, VarError},
    smt::Error,
};

pub mod config;
pub mod models;
pub mod schema;
pub mod store;
