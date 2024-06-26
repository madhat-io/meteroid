use crate::errors::IntoDbResult;
use crate::tenants::{Tenant, TenantNew};
use crate::{DbResult, PgConn};

use diesel::debug_query;
use diesel::prelude::{ExpressionMethods, QueryDsl};
use error_stack::ResultExt;

impl TenantNew {
    pub async fn insert(&self, conn: &mut PgConn) -> DbResult<Tenant> {
        use crate::schema::tenant::dsl::*;
        use diesel_async::RunQueryDsl;

        let query = diesel::insert_into(tenant).values(self);
        log::debug!("{}", debug_query::<diesel::pg::Pg, _>(&query).to_string());

        query
            .get_result(conn)
            .await
            .attach_printable("Error while inserting tenant")
            .into_db_result()
    }
}

impl Tenant {
    pub async fn find_by_id(conn: &mut PgConn, tenant_id: uuid::Uuid) -> DbResult<Tenant> {
        use crate::schema::tenant::dsl::*;
        use diesel_async::RunQueryDsl;

        let query = tenant.filter(id.eq(tenant_id));
        log::debug!("{}", debug_query::<diesel::pg::Pg, _>(&query).to_string());

        query
            .first(conn)
            .await
            .attach_printable("Error while finding tenant by id")
            .into_db_result()
    }
}
