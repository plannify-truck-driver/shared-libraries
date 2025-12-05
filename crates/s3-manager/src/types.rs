#[derive(Debug, Clone, PartialEq)]
pub struct ObjectMetadata {
    pub key: String,
    pub size: u64,
    pub e_tag: String,
}

pub enum ObjectType {
    DriverMonthlyWorkdayReport,
    DriverMailAttachment,
}

impl ObjectType {
    pub fn to_s3_path(&self, entity_id: &str) -> String {
        match self {
            ObjectType::DriverMonthlyWorkdayReport => format!("drivers/{}/monthly_workday_reports", entity_id),
            ObjectType::DriverMailAttachment => format!("drivers/{}/mail_attachments", entity_id),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ObjectType::DriverMonthlyWorkdayReport => "DRIVER_MONTHLY_WORKDAY_REPORT".to_string(),
            ObjectType::DriverMailAttachment => "DRIVER_MAIL_ATTACHMENT".to_string(),
        }
    }
}