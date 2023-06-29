use diesel::{prelude::*, AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::{
    config::db::Connection,
    models::pagination::SortingAndPaging,
    schema::people::{self, dsl::*},
};

use super::{filters::PersonFilter, response::Page};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub gender: bool,
    pub age: i32,
    pub address: String,
    pub phone: String,
    pub email: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = people)]
pub struct PersonDTO {
    pub name: String,
    pub gender: bool,
    pub age: i32,
    pub address: String,
    pub phone: String,
    pub email: String,
}

impl Person {
    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<Person>> {
        people.order(id.asc()).load::<Person>(conn)
    }

    pub fn find_by_id(i: i32, conn: &mut Connection) -> QueryResult<Person> {
        people.find(i).get_result::<Person>(conn)
    }

    pub fn filter(filter: PersonFilter, conn: &mut Connection) -> QueryResult<Page<Person>> {
        let mut query = people::table.into_boxed();

        if let Some(i) = filter.age {
            query = query.filter(age.eq(i));
        }
        if let Some(i) = filter.email {
            query = query.filter(email.like(format!("%{}%", i)));
        }
        if let Some(i) = filter.gender {
            match i.to_lowercase().as_str() {
                "male" => {
                    query = query.filter(gender.eq(true));
                }
                "female" => {
                    query = query.filter(gender.eq(false));
                }
                _ => {}
            }
        }
        if let Some(i) = filter.name {
            query = query.filter(name.like(format!("%{}%", i)));
        }
        if let Some(i) = filter.phone {
            query = query.filter(phone.like(format!("%{}%", i)));
        }

        query
            .paginate(
                filter
                    .page_num
                    .unwrap_or(crate::constants::DEFAULT_PAGE_NUM),
            )
            .per_page(
                filter
                    .page_size
                    .unwrap_or(crate::constants::DEFAULT_PER_PAGE),
            )
            .sort(
                filter
                    .sort_by
                    .unwrap_or(crate::constants::EMPTY_STR.to_string()),
                filter
                    .sort_direction
                    .unwrap_or(crate::constants::EMPTY_STR.to_string()),
            )
            .load_and_count_items::<Person>(conn)
    }

    pub fn insert(new_person: PersonDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(people)
            .values(&new_person)
            .execute(conn)
    }

    pub fn update(i: i32, updated_person: PersonDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::update(people.find(i))
            .set(&updated_person)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(people.find(i)).execute(conn)
    }
}
