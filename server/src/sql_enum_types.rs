use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(SqlType)]
#[postgres(type_name = "progress")]
pub struct Progress;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[sql_type = "Progress"]
pub enum ProgressEnum {
    NotStarted,
    InProgress,
    Finished
}

// From example: https://github.com/ebkalderon/diesel/blob/db1a5156a7224ca978da806825efbfc3f349c558/diesel_tests/tests/custom_types.rs
impl ToSql<Progress, Pg> for ProgressEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            ProgressEnum::NotStarted => out.write_all(b"not_started")?,
            ProgressEnum::InProgress => out.write_all(b"in_progress")?,
            ProgressEnum::Finished => out.write_all(b"finished")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Progress, Pg> for ProgressEnum {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"not_started" => Ok(ProgressEnum::NotStarted),
            b"in_progress" => Ok(ProgressEnum::InProgress),
            b"finished" => Ok(ProgressEnum::Finished),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}