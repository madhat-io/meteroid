use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::enums::{BillingMetricAggregateEnum, UnitConversionRoundingEnum};

use diesel::{Identifiable, Insertable, Queryable};

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = crate::schema::billable_metric)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BillableMetric {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub code: String,
    pub aggregation_type: BillingMetricAggregateEnum,
    pub aggregation_key: Option<String>,
    pub unit_conversion_factor: Option<i32>,
    pub unit_conversion_rounding: Option<UnitConversionRoundingEnum>,
    pub segmentation_matrix: Option<serde_json::Value>,
    pub usage_group_key: Option<String>,
    pub created_at: NaiveDateTime,
    pub created_by: Uuid,
    pub updated_at: Option<NaiveDateTime>,
    pub archived_at: Option<NaiveDateTime>,
    pub tenant_id: Uuid,
    pub product_family_id: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::billable_metric)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BillableMetricNew {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub code: String,
    pub aggregation_type: BillingMetricAggregateEnum,
    pub aggregation_key: Option<String>,
    pub unit_conversion_factor: Option<i32>,
    pub unit_conversion_rounding: Option<UnitConversionRoundingEnum>,
    pub segmentation_matrix: Option<serde_json::Value>,
    pub usage_group_key: Option<String>,
    pub created_by: Uuid,
    pub tenant_id: Uuid,
    pub product_family_id: Uuid,
}
