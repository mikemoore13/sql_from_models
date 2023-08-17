use sql_from_models_parser::ast::DataType;
#[cfg(feature = "serde")]
use serde::*;
use std::ops::{Deref, DerefMut};

use crate::prelude::*;


/// PostgreSQL `BigSerial` type. It enables autoincrementing functionality. 
/// Example: 
/// ```
/// struct Profile {
///     id: BigSerial,
/// }
/// ```
/// The previus structure would generate: 
/// ```sql
/// CREATE TABLE profile (
///     id BigSerial NOT NULL
/// );
/// ```
/// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BigSerial(pub i64);

impl<T> From<T> for BigSerial
where
    T: Into<i64>,
{
    fn from(obj: T) -> Self {
        Self(obj.into())
    }
}

impl Deref for BigSerial {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BigSerial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsMut<i64> for BigSerial {
    fn as_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

impl AsRef<i64> for BigSerial {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl IntoSQL for BigSerial {
    fn into_sql() -> DataType {
        DataType::BigSerial
    }
}


