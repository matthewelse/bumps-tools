use std::fmt::Display;

use clap::{Parser, ValueEnum};
use diesel::{
    backend::Backend,
    deserialize::FromSql,
    prelude::*,
    serialize::{Output, ToSql},
    sql_types::VarChar,
    sqlite::Sqlite,
    AsExpression, FromSqlRow,
};

#[derive(Debug, FromSqlRow, AsExpression, Parser, PartialEq, Clone, ValueEnum, Copy)]
#[diesel(sql_type = diesel::sql_types::VarChar)]
pub(crate) enum Competition {
    Early,
    MenMays,
    WomenMays,
    MenLents,
    WomenLents,
}

impl FromSql<VarChar, Sqlite> for Competition {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        Ok(Competition::from_slug(<String as FromSql<VarChar, Sqlite>>::from_sql(bytes)?.as_str())?)
    }
}

impl ToSql<VarChar, Sqlite> for Competition {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        let result = <str as ToSql<diesel::sql_types::VarChar, Sqlite>>::to_sql(self.slug(), out)?;
        Ok(result)
    }
}

impl Competition {
    pub(crate) fn from_slug(s : &str) -> Result<Self, String> {
        match s {
            "early" => Ok(Self::Early),
            "mmays" => Ok(Self::MenMays),
            "wmays" => Ok(Self::WomenMays),
            "mlents" => Ok(Self::MenLents),
            "wlents" => Ok(Self::WomenLents),
            name => Err(format!("Invalid competition name {name}").into()),
        }
    }

    pub(crate) fn slug(&self) -> &'static str {
        match self {
            Self::Early => "early",
            Self::MenMays => "mmays",
            Self::WomenMays => "wmays",
            Self::MenLents => "mlents",
            Self::WomenLents => "wlents",
        }
    }

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

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::entries)]
pub(crate) struct NewEntry<'a> {
    pub year: i32,
    pub day: i32,
    pub club: &'a str,
    pub crew: &'a str,
    pub competition: Competition,
    pub position: i32,
}
