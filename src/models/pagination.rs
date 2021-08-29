// Source: https://github.com/diesel-rs/diesel/blob/v1.4.4/examples/postgres/advanced-blog-cli/src/pagination.rs

#![allow(unused_imports)]
use super::Page;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;
use diesel::sqlite::Sqlite;

pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: crate::constants::DEFAULT_PER_PAGE,
            page,
        }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
}

#[cfg(not(test))]
impl<T> Paginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated { per_page, ..self }
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
        Ok(Page {
            data: records,
            curr_page_num: page,
            page_size: per_page,
            total_elements: total,
        })
    }
}

#[cfg(test)]
impl<T> Paginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated { per_page, ..self }
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
        Ok(Page {
            data: records,
            curr_page_num: page,
            page_size: per_page,
            total_elements: total,
        })
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

#[cfg(not(test))]
impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

#[cfg(not(test))]
impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.per_page;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}

#[cfg(test)]
impl<T> RunQueryDsl<SqliteConnection> for Paginated<T> {}

#[cfg(test)]
impl<T> QueryFragment<Sqlite> for Paginated<T>
where
    T: QueryFragment<Sqlite>,
{
    fn walk_ast(&self, mut out: AstPass<Sqlite>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.per_page;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}
