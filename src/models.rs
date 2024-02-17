use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Package {
    pub name: String,
    pub version: String,
    // dependecies: Vec<String>,
    pub size: String,
}
