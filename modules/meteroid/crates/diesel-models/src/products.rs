use chrono::NaiveDateTime;
use uuid::Uuid;

use diesel::{Identifiable, Insertable, Queryable};

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::product)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: Uuid,
    pub updated_at: Option<NaiveDateTime>,
    pub archived_at: Option<NaiveDateTime>,
    pub tenant_id: Uuid,
    pub product_family_id: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::product)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductNew {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub tenant_id: Uuid,
    pub product_family_id: Uuid,
}
