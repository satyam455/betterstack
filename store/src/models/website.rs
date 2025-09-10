use crate::schema::website::*;
use crate::store::Store;
use chrono::DateTime;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Website {
    pub id: String,
    pub url: String,
    pub userId: String,
    pub timeAdded: NaiveDateTime,
}

impl Store {
    pub fn create_website(
        &mut self,
        user_Id: String,
        input_url: String,
    ) -> Result<Website, diesel::result::Error> {
        let get_id = Uuid::new_v4().to_string();
        // let time_added = chrono::Utc::now().naive_utc();
        let w = Website {
            id: get_id,
            url: input_url,
            userId: user_Id,
            timeAdded: chrono::Utc::now().naive_utc(),
        };

        let website = diesel::insert_into(crate::schema::website::table)
            .values(&w)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)?;

        Ok(website)
    }

    pub fn get_website(&mut self, input_id: String) -> Result<Website, diesel::result::Error> {
        use crate::schema::website::dsl::*;

        let website_result = website
            .filter(id.eq(input_id))
            .select(Website::as_select())
            .first(&mut self.conn)?;

        Ok(website_result)
    }
}
