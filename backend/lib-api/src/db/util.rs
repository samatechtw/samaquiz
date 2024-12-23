use std::fmt::Display;

use sqlx::{Database, Encode, Postgres, QueryBuilder, Transaction, Type};
use strum::{Display, EnumString};
use uuid::Uuid;

use crate::error::api_error::ApiError;

pub fn append_limit_offset<'a>(
    mut query: QueryBuilder<'a, Postgres>,
    from: i32,
    to: i32,
) -> QueryBuilder<'a, Postgres> {
    query.push(" LIMIT ");
    let limit = to - from + 1;
    query.push_bind(limit as i32);
    query.push(" OFFSET ");
    query.push_bind((from - 1) as i32);
    query
}

/// Appends a " field LIKE %$x%" binding to the query builder,
/// prepended with " AND" if it's not the first call
/// Returns the input `count + 1` if a parameter was bound
pub fn append_like<
    'a,
    T: 'a + ToString + sqlx::Type<Postgres> + sqlx::Encode<'a, Postgres> + Send,
>(
    query: QueryBuilder<'a, Postgres>,
    arg_name: &str,
    arg: Option<T>,
    count: u32,
) -> (QueryBuilder<'a, Postgres>, u32) {
    if let Some(arg_val) = arg {
        let (mut q, count) = append_op(query, DbOp::And, count);
        q.push(format!(" lower(\"{}\") LIKE ", arg_name));
        q.push_bind(format!("%{}%", arg_val.to_string().to_lowercase()));
        return (q, count + 1);
    }
    (query, count)
}

// Commits or rolls back an SQLx transaction based on success or failure of
// an external APi call
pub async fn commit_or_rollback<T>(
    tx: Transaction<'_, Postgres>,
    result: Result<T, ApiError>,
) -> Result<T, ApiError> {
    match result {
        Err(e) => Err(match tx.rollback().await {
            Err(rollback_err) => ApiError::internal_error().message(rollback_err.to_string()),
            Ok(_) => ApiError::internal_error().message(e),
        }),
        Ok(val) => {
            // Commit the transaction
            tx.commit()
                .await
                .map_err(|e| ApiError::internal_error().message(e))?;
            Ok(val)
        }
    }
}

#[derive(Debug, Display, PartialEq, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum DbOp {
    And,
    Or,
}

pub fn quote(sql: &str) -> String {
    format!("\"{sql}\"")
}

/// Appends " <op>" if count is greater than 0
pub fn append_op<'a, DB: Database>(
    mut query: QueryBuilder<'a, DB>,
    op: DbOp,
    count: u32,
) -> (QueryBuilder<'a, DB>, u32) {
    if count != 0 {
        query.push(format!(" {}", op));
    }
    (query, count + 1)
}

/// Appends a " field = $x" binding to the query builder,
/// prepended with " AND" if it's not the first call
/// Returns the input `count + 1` if a parameter was bound
pub fn append_eq<
    'a,
    DB: Database,
    T: 'a + ToString + sqlx::Type<DB> + sqlx::Encode<'a, DB> + Send,
>(
    query: QueryBuilder<'a, DB>,
    op: DbOp,
    arg_name: &str,
    arg: Option<T>,
    count: u32,
) -> (QueryBuilder<'a, DB>, u32) {
    if let Some(arg_val) = arg {
        let (mut q, count) = append_op(query, op, count);
        q.push(format!(" \"{}\" = ", arg_name));
        q.push_bind(arg_val);
        return (q, count + 1);
    }
    (query, count)
}

pub fn append_and_eq<
    'a,
    DB: Database,
    T: 'a + ToString + sqlx::Type<DB> + sqlx::Encode<'a, DB> + Send,
>(
    query: QueryBuilder<'a, DB>,
    arg_name: &str,
    arg: Option<T>,
    count: u32,
) -> (QueryBuilder<'a, DB>, u32) {
    append_eq(query, DbOp::And, arg_name, arg, count)
}

pub fn append_or_eq<
    'a,
    DB: Database,
    T: 'a + ToString + sqlx::Type<DB> + sqlx::Encode<'a, DB> + Send,
>(
    query: QueryBuilder<'a, DB>,
    arg_name: &str,
    arg: Option<T>,
    count: u32,
) -> (QueryBuilder<'a, DB>, u32) {
    append_eq(query, DbOp::Or, arg_name, arg, count)
}

fn push_comma<'a, DB: sqlx::Database>(
    mut query: QueryBuilder<'a, DB>,
    arg_name: &str,
    count: u32,
) -> QueryBuilder<'a, DB> {
    if count == 0 {
        query.push(format!(" {} = ", arg_name));
    } else {
        query.push(format!(", {} = ", arg_name));
    }
    query
}

// Appends a comma separated value to the query.
// Option/None is accepted, which converts to NULL
pub fn append_nullable_comma<'a, T: 'a + Encode<'a, DB> + Type<DB>, DB: sqlx::Database>(
    mut query: QueryBuilder<'a, DB>,
    arg_name: &str,
    arg: T,
    count: u32,
) -> (QueryBuilder<'a, DB>, u32) {
    query = push_comma(query, arg_name, count);
    query.push_bind(arg);
    (query, count + 1)
}

// Appends a non-null comma separated value to the query.
pub fn append_comma<'a, T: 'a + Encode<'a, DB> + Type<DB>, DB: sqlx::Database>(
    query: QueryBuilder<'a, DB>,
    arg_name: &str,
    arg: Option<T>,
    count: u32,
) -> (QueryBuilder<'a, DB>, u32) {
    if let Some(arg_val) = arg {
        return append_nullable_comma(query, arg_name, arg_val, count);
    }
    (query, count)
}

pub fn append_order_by<'a, DB: Database>(
    mut query: QueryBuilder<'a, DB>,
    column: String,
    direction: String,
) -> QueryBuilder<'a, DB> {
    query.push(format!(" ORDER BY {}", column));
    query.push(format!(" {}", direction));
    query
}

pub fn append_array_contains<'a, DB, T>(
    query: QueryBuilder<'a, DB>,
    arg_name: &str,
    arg: Option<T>,
    count: u32,
) -> (QueryBuilder<'a, DB>, u32)
where
    DB: Database,
    T: 'a + sqlx::Type<DB> + sqlx::Encode<'a, DB> + Send,
{
    if let Some(arg_val) = arg {
        let (mut q, count) = append_op(query, DbOp::And, count);
        q.push(" ");
        q.push_bind(arg_val);
        q.push(format!(" <@ {}", arg_name));
        return (q, count + 1);
    }
    (query, count)
}

/// Appends array comparison using "ANY" operator
pub fn append_in<
    'a,
    T: 'a + Display + ToString + sqlx::Type<Postgres> + sqlx::Encode<'a, Postgres> + Send,
>(
    query: QueryBuilder<'a, Postgres>,
    field: &str,
    compare: Option<Vec<T>>,
    count: u32,
) -> (QueryBuilder<'a, Postgres>, u32) {
    if let Some(compare_val) = compare {
        let (mut q, count) = append_op(query, DbOp::And, count);
        q.push(format!(" \"{}\" IN (", field));
        let mut c2 = 0;
        for item in compare_val.iter() {
            if c2 != 0 {
                q.push(", ");
            }
            q.push_bind(format!("{}", item));
            c2 += 1;
        }
        q.push(")");
        return (q, count + 1);
    }
    (query, count)
}

pub fn option_string_to_uuid(str: Option<String>) -> Option<Uuid> {
    str.and_then(|id| Uuid::parse_str(&id).ok())
}

pub fn option_enum_to_string<T: ToString>(value: Option<T>) -> Option<String> {
    value.and_then(|v| Some(v.to_string()))
}
