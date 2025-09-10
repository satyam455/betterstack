// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "websiteStatus"))]
    pub struct WebsiteStatus;
}

diesel::table! {
    Region (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    User (id) {
        id -> Text,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    website (id) {
        id -> Text,
        url -> Text,
        timeAdded -> Timestamp,
        userId -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebsiteStatus;

    websiteTick (id) {
        id -> Text,
        response_time_ms -> Int4,
        status -> WebsiteStatus,
        region_id -> Text,
        website_id -> Text,
        createdAt -> Timestamp,
    }
}

diesel::joinable!(website -> User (userId));
diesel::joinable!(websiteTick -> Region (region_id));
diesel::joinable!(websiteTick -> website (website_id));

diesel::allow_tables_to_appear_in_same_query!(
    Region,
    User,
    website,
    websiteTick,
);
