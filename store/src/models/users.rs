use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::User)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

impl Store {
    pub fn sign_up(
        &mut self,
        input_username: String,
        input_password: String,
    ) -> Result<String, diesel::result::Error> {
        let u = User {
            id: Uuid::new_v4().to_string(),
            username: input_username,
            password: input_password,
        };

        let result = diesel::insert_into(crate::schema::User::table)
            .values(&u)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;

        Ok(result.id.to_string())
    }

    pub fn sign_in(
        &mut self,
        input_username: String,
        input_password: String,
    ) -> Result<bool, diesel::result::Error> {
        use crate::schema::User::dsl::*;

        let user_result = User
            .filter(username.eq(input_username))
            .select(crate::models::users::User::as_select())
            .first(&mut self.conn)?;

        if user_result.password != input_password {
            return Ok(false);
        }

        Ok(true)
    }
}
