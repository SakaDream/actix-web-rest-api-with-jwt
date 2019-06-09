use crate::{
    config::db::Connection,
    schema::people::{self, dsl::*}
};
use diesel::prelude::*;

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
#[table_name = "people"]
pub struct PersonDTO {
    pub name: String,
    pub gender: bool,
    pub age: i32,
    pub address: String,
    pub phone: String,
    pub email: String,
}

impl Person {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Person>> {
        people.order(id.asc()).load::<Person>(conn)
    }

    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Person> {
        people.find(i).get_result::<Person>(conn)
    }

    pub fn query(query: String, conn: &Connection) -> QueryResult<Vec<Person>> {
        let pattern = format!("%{}%", query);
        let mut id_and_age_query: i32 = 0;
        let mut id_and_age_query_flag = false;
        if query.as_str().parse::<i32>().is_ok() {
            id_and_age_query_flag = true;
            id_and_age_query = query.as_str().parse::<i32>().unwrap();
        }

        let gender_query;
        let gender_query_flag;
        match query.to_lowercase().as_str() {
            "male" => {
                gender_query = true;
                gender_query_flag = true;
            }
            "female" => {
                gender_query = false;
                gender_query_flag = true;
            }
            _ => {
                gender_query = false;
                gender_query_flag = false;
            }
        }

        if id_and_age_query_flag && gender_query_flag {
            people
                .order(id.asc())
                .filter(id.eq(&id_and_age_query))
                .or_filter(name.like(&pattern))
                .or_filter(gender.eq(&gender_query))
                .or_filter(age.eq(&id_and_age_query))
                .or_filter(address.like(&pattern))
                .load::<Person>(conn)
        } else if id_and_age_query_flag && !gender_query_flag {
            people
                .order(id.asc())
                .filter(id.eq(&id_and_age_query))
                .or_filter(name.like(&pattern))
                .or_filter(age.eq(&id_and_age_query))
                .or_filter(address.like(&pattern))
                .load::<Person>(conn)
        } else if !id_and_age_query_flag && gender_query_flag {
            people
                .order(id.asc())
                .filter(name.like(&pattern))
                .or_filter(gender.eq(&gender_query))
                .or_filter(address.like(&pattern))
                .load::<Person>(conn)
        } else {
            people
                .order(id.asc())
                .filter(name.like(&pattern))
                .or_filter(address.like(&pattern))
                .load::<Person>(conn)
        }
    }

    pub fn insert(new_person: PersonDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(people)
            .values(&new_person)
            .execute(conn)
    }

    pub fn update(i: i32, updated_person: PersonDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::update(people.find(i))
            .set(&updated_person)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(people.find(i)).execute(conn)
    }
}
