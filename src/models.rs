use chrono::prelude::*;
use sea_query::{ColumnDef, Expr, Func, Iden, OnConflict, Order, Query, SqliteQueryBuilder, Table};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Pkg {
    pub name: String,
    pub version: String,
    // dependecies: Vec<String>,
    pub size: String,
}

#[derive(Iden)]
pub enum Packages {
    Pkg,
    Name,
    Table,
    Version,
    Size,
}
