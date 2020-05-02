use rusqlite::ToSql;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};

use crate::units::Unit;

impl ToSql for Unit {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        let value = match self {
            Unit::None => 0,
            Unit::Kilograms => 1,
            Unit::Liters => 2,
        };

        Ok(ToSqlOutput::Borrowed(ValueRef::Integer(value)))
    }
}

impl FromSql for Unit {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(0) => Ok(Unit::None),
            ValueRef::Integer(1) => Ok(Unit::Kilograms),
            ValueRef::Integer(2) => Ok(Unit::Liters),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}