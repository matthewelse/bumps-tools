use std::fmt::Display;

use clap::{Parser, ValueEnum};
use diesel::{
    backend::Backend, deserialize::FromSql, prelude::*, serialize::{ToSql, Output}, sqlite::Sqlite,
    AsExpression, FromSqlRow,
};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(
    Debug,
    FromSqlRow,
    AsExpression,
    Parser,
    PartialEq,
    Clone,
    ValueEnum,
    FromPrimitive,
    ToPrimitive,
    Copy,
)]
#[diesel(sql_type = diesel::sql_types::Integer)]
pub(crate) enum Competition {
    Early,
    MenMays,
    WomenMays,
    MenLents,
    WomenLents,
}

impl FromSql<diesel::sql_types::Integer, Sqlite> for Competition {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(Self::Early),
            1 => Ok(Self::MenMays),
            2 => Ok(Self::WomenMays),
            3 => Ok(Self::MenLents),
            4 => Ok(Self::WomenLents),
            i => Err(format!("Unrecognised enum variant: {i}").into()),
        }
    }
}

impl ToSql<diesel::sql_types::Integer, Sqlite> for Competition {
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        let val = Box::new(self.to_i32().unwrap());
        let result = <i32 as ToSql<diesel::sql_types::Integer, Sqlite>>::to_sql(Box::leak(val), out)?;
        Ok(result)
    }
}

impl Competition {
    pub(crate) fn raw_name(&self) -> &'static str {
        match self {
            Self::Early => "early",
            Self::MenMays => "mays",
            Self::WomenMays => "wmays",
            Self::MenLents => "lents",
            Self::WomenLents => "wlents",
        }
    }

    pub(crate) fn charts_name(&self) -> &'static str {
        match self {
            Self::Early => "Early",
            Self::MenMays => "Mays",
            Self::WomenMays => "WMays",
            Self::MenLents => "Lents",
            Self::WomenLents => "WLents",
        }
    }
}

impl Display for Competition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Early => "early bumps",
            Self::MenMays => "men's may bumps",
            Self::WomenMays => "women's may bumps",
            Self::MenLents => "men's may bumps",
            Self::WomenLents => "women's may bumps",
        };

        f.write_str(text)
    }
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Entry {
    id : i32,
    club : String,
    crew: String,
    year: i32,
    day: i32,
    position : i32,
    competition: Competition,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::entries)]
pub(crate) struct NewEntry<'a> {
    pub year : i32,
    pub day : i32,
    pub club : &'a str,
    pub crew : &'a str,
    pub competition : Competition,
    pub position : i32
}
