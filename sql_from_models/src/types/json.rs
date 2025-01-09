use super::*;
use sql_from_models_parser::ast::DataType;
use serde::*;
use std::ops::{Deref, DerefMut};

/// Wrapper type used to hold serializable data. The type generated is `JSON`.
/// ```rust
/// use sql_from_models::Json;
/// struct Author {
///     books: Json<Vec<String>>
/// }
/// ```
/// The previous structure would generate:
/// ```sql
/// CREATE TABLE author (
///     books JSON NOT NULL,
/// );
/// ```

#[derive(Serialize, Deserialize, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Json<T>(pub T);

impl<T> Deref for Json<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for Json<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Json<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> IntoSQL for Json<T> {
    fn into_sql() -> DataType {
        DataType::Json
    }
    const IS_NULLABLE: bool = false;
}
#[allow(unused_imports)]
#[cfg(feature = "sqlx")]
mod sqlx_impl {
    use super::*;
    use serde::{Deserialize, Serialize};
    #[cfg(feature = "sqlx-mysql")]
    use sqlx::mysql::{MySql, MySqlTypeInfo};
    #[cfg(feature = "sqlx-sqlite")]
    use sqlx::sqlite::{Sqlite, SqliteTypeInfo};
    #[cfg(feature = "sqlx-postgres")]
    use sqlx::postgres::{PgTypeInfo, Postgres};
    use sqlx::{
        decode::Decode,
        encode::{Encode, IsNull},
        types::Json as SqlxJson,
        Database, Type,
    };

    /// Implement `Type` trait for `Json` to map it to the database's JSON type.
    impl<T, DB> Type<DB> for Json<T>
    where
        DB: Database,
        SqlxJson<T>: Type<DB>,
    {
        fn type_info() -> DB::TypeInfo {
            SqlxJson::<T>::type_info()
        }

        fn compatible(ty: &DB::TypeInfo) -> bool {
            SqlxJson::<T>::compatible(ty)
        }
    }

    /// Implement `Encode` for `Json` to allow it to be written to the database.
    impl<'q, T, DB> Encode<'q, DB> for Json<T>
    where
        DB: Database,
        T: Serialize + Clone,
        SqlxJson<T>: Encode<'q, DB>,
    {
        fn encode_by_ref(&self, buf: &mut <DB as sqlx::Database>::ArgumentBuffer<'q>) -> Result<IsNull, Box<dyn std::error::Error + 'static + Send + Sync>> {
            <SqlxJson<T> as Encode<'q, DB>>::encode_by_ref(&SqlxJson(self.0.clone()), buf)
        }
    }

    /// Implement `Decode` for `Json` to allow it to be read from the database.
    impl<'r, DB, T> Decode<'r, DB> for Json<T>
    where
        DB: Database,
        SqlxJson<T>: Decode<'r, DB>,
        T: Deserialize<'r>,
    {
        fn decode(
            value: <DB as sqlx::Database>::ValueRef<'r>,
        ) -> Result<Json<T>, Box<dyn std::error::Error + 'static + Send + Sync>> {
            let json = SqlxJson::<T>::decode(value)?;
            Ok(Json(json.0))
        }
    }
}
