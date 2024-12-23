use lib_api::db::db_error::DbError;
use lib_types::type_util::{is_array, is_bool, is_date, is_float, is_number, is_uuid};
use serde::{ser, Serialize};
use serde_json::{to_value, Error};
use sqlx::{PgPool, Postgres, QueryBuilder};

pub fn struct_fields<T: Serialize>(items: &Vec<T>) -> Result<Vec<String>, Error> {
    let serde_value = to_value(items.get(0))?;
    let map = serde_value
        .as_object()
        .ok_or(ser::Error::custom("Failed to get fixture fields"))?;
    Ok(map.keys().map(|k| k.to_string()).collect())
}

pub fn struct_values<T: Serialize>(
    fields: &Vec<String>,
    items: &Vec<T>,
) -> Result<Vec<Vec<String>>, Error> {
    let result = items
        .into_iter()
        .map(|item| {
            let item_map = to_value(item)?;
            item_map
                .as_object()
                .ok_or(ser::Error::custom("Failed to create fixture map"))?;
            Ok(fields
                .into_iter()
                .map(|f| {
                    let item = item_map
                        .get(f)
                        .ok_or(ser::Error::custom("Fixture value missing"))?;
                    if item.is_boolean() {
                        let b = item
                            .as_bool()
                            .ok_or(ser::Error::custom("Fixture boolean invalid"))?;
                        return Ok((if b { "__true__" } else { "__false__" }).into());
                    }
                    if item.is_i64() {
                        let i = item.as_i64().unwrap(); // Assumes valid i64 values
                        return Ok(i.to_string().into());
                    }
                    if item.is_f64() {
                        let f = item.as_f64().unwrap(); // Assumes valid i64 values
                        return Ok(f.to_string().into());
                    }
                    if item.is_null() {
                        return Ok("NULL".into());
                    }
                    if item.is_array() {
                        let a = item
                            .as_array()
                            .ok_or(ser::Error::custom("Fixture array invalid"))?;

                        let values: Vec<String> = a
                            .iter()
                            .map(|array_item| {
                                if array_item.is_string() {
                                    return Ok(array_item.as_str().unwrap().to_string());
                                } else {
                                    return Err(ser::Error::custom("Invalid fixture array item"));
                                }
                            })
                            .collect::<Result<Vec<String>, _>>()?;

                        let array_value = serde_json::to_value(values)?;

                        return Ok(array_value.to_string());
                    }
                    let val_str = item
                        .as_str()
                        .ok_or(ser::Error::custom(format!(
                            "Fixture value invalid: {:?}",
                            item
                        )))?
                        .to_string();
                    Ok(format!("__STR__{}", val_str.to_string()))
                })
                .collect::<Result<Vec<String>, _>>()?)
        })
        .collect::<Result<Vec<Vec<String>>, _>>()?;

    Ok(result)
}

pub async fn bulk_insert<T: Serialize>(
    db: &PgPool,
    table: &str,
    items: &Vec<T>,
) -> Result<(), DbError> {
    let fields = struct_fields(items).map_err(|e| DbError::Serialize(e.to_string()))?;

    let values = struct_values(&fields, items).map_err(|e| DbError::Serialize(e.to_string()))?;

    // For debugging serialized values
    /*
    let values_str = values
        .clone()
        .into_iter()
        .map(|arr| arr.join(", "))
        .collect::<Vec<String>>()
        .join("),(");
    println!("VALUES: ({})", values_str);
    */

    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new(format!("INSERT INTO {}({}) ", table, fields.join(", ")));

    query_builder.push_values(values.into_iter(), |mut b, values| {
        for mut val in values.into_iter() {
            // TODO -- I couldn't figure out how to get SQLx to include the correct type, since we're inserting
            // values which have already been serialized. Checking the values with a regex is annoying, but seems to work.
            if val == "NULL" {
                b.push("NULL");
            } else if is_float(&val) {
                b.push_bind(val);
                b.push_unseparated("::double precision ");
            } else if is_number(&val) {
                if val.parse::<i64>().unwrap() > i32::MAX.into() {
                    b.push_bind(val);
                    b.push_unseparated("::bigint");
                } else {
                    b.push_bind(val);
                    b.push_unseparated("::integer");
                }
            } else if is_bool(&val) {
                b.push_bind(if val == "__true__" { "t" } else { "f" });
                b.push_unseparated("::boolean");
            } else if is_array(&val) {
                b.push_bind(val.replace("[", "{").replace("]", "}"));
                b.push_unseparated("::text[]");
            } else if val.starts_with("__DEC__") {
                b.push_bind(val.strip_prefix("__DEC__").unwrap().to_string());
                b.push_unseparated("::decimal");
            } else {
                // Check string types
                if val.starts_with("__STR__") {
                    val = val.strip_prefix("__STR__").unwrap().to_string()
                }
                if is_date(&val) {
                    b.push_bind(val);
                    b.push_unseparated("::timestamptz");
                } else if is_uuid(&val) {
                    b.push_bind(val);
                    b.push_unseparated("::uuid");
                } else {
                    b.push_bind(val);
                }
            }
        }
    });

    query_builder
        .build()
        .execute(db)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

    Ok(())
}
