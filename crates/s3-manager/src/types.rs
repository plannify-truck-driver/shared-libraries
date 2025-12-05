#[derive(Debug, Clone, PartialEq)]
pub struct ObjectMetadata {
    pub key: String,
    pub size: u64,
    pub e_tag: String,
}

pub enum ObjectType {
    DriverMonthlyWorkdayReport
}

impl ObjectType {
    pub fn to_s3_path(&self, entity_id: &str) -> String {
        match self {
            ObjectType::DriverMonthlyWorkdayReport => format!("drivers/{}/monthly_workday_reports", entity_id),
        }
    }
}