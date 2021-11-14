// Source: https://github.com/diesel-rs/diesel/blob/master/examples/postgres/advanced-blog-cli/src/pagination.rs

#![allow(unused_imports)]
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;
use diesel::sqlite::Sqlite;

use crate::constants::MESSAGE_OK;

use super::response::Page;

pub trait SortingAndPaging: Sized {
    fn paginate(self, page: i64) -> SortedAndPaginated<Self>;
}

impl<T> SortingAndPaging for T {
    fn paginate(self, page: i64) -> SortedAndPaginated<Self> {
        SortedAndPaginated {
            query: self,
            sort_by: crate::constants::EMPTY_STR.to_string(),
            sort_direction: crate::constants::EMPTY_STR.to_string(),
            per_page: crate::constants::DEFAULT_PER_PAGE,
            page,
        }
    }
}

#[derive(Debug, Clone, QueryId)]
pub struct SortedAndPaginated<T> {
    query: T,
    sort_by: String,
    sort_direction: String,
    page: i64,
    per_page: i64,
}

#[cfg(not(test))]
impl<T> SortedAndPaginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        SortedAndPaginated { per_page, ..self }
    }

    pub fn sort(self, sort_by: String, sort_direction: String) -> Self {
        SortedAndPaginated {
            sort_by,
            sort_direction,
            ..self
        }
    }

    pub fn load_and_count_items<U>(self, conn: &PgConnection) -> QueryResult<Page<U>>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let page = self.page;
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        Ok(Page::new(MESSAGE_OK, records, page, per_page, total))
    }
}

#[cfg(test)]
impl<T> SortedAndPaginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        SortedAndPaginated { per_page, ..self }
    }

    pub fn sort(self, sort_by: String, sort_direction: String) -> Self {
        SortedAndPaginated {
            sort_by,
            sort_direction,
            ..self
        }
    }

    pub fn load_and_count_items<U>(self, conn: &SqliteConnection) -> QueryResult<Page<U>>
    where
        Self: LoadQuery<SqliteConnection, (U, i64)>,
    {
        let page = self.page;
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        Ok(Page::new(MESSAGE_OK, records, page, per_page, total))
    }
}

impl<T: Query> Query for SortedAndPaginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

#[cfg(not(test))]
impl<T> RunQueryDsl<PgConnection> for SortedAndPaginated<T> {}

#[cfg(not(test))]
impl<T> QueryFragment<Pg> for SortedAndPaginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t ");
        if &self.sort_by.as_str().len() > &0 {
            out.push_sql(format!(" ORDER BY {} {}", &self.sort_by, &self.sort_direction).as_str());
        }
        out.push_sql(" LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.per_page;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}

#[cfg(test)]
impl<T> RunQueryDsl<SqliteConnection> for SortedAndPaginated<T> {}

#[cfg(test)]
impl<T> QueryFragment<Sqlite> for SortedAndPaginated<T>
where
    T: QueryFragment<Sqlite>,
{
    fn walk_ast(&self, mut out: AstPass<Sqlite>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t ");
        if &self.sort_by.as_str().len() > &0 {
            out.push_sql(format!(" ORDER BY {} {}", &self.sort_by, &self.sort_direction).as_str());
        }
        out.push_sql(" LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.per_page;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}
