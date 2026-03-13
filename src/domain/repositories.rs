use crate::domain::entities::Attendance;
use crate::domain::errors::DomainError;

pub trait AttendanceRepository {
    fn create(&self, attendance: &Attendance) -> Result<(), DomainError>;
    fn update(&self, attendance: &Attendance) -> Result<(), DomainError>;
    fn delete(&self, id: &str) -> Result<(), DomainError>;
    fn find_by_id(&self, id: &str) -> Result<Attendance, DomainError>;
    fn find_all(&self) -> Result<Vec<Attendance>, DomainError>;
    fn find_by_date_range(&self, start: &str, end: &str) -> Result<Vec<Attendance>, DomainError>;
    fn find_overdue(&self, today: &str) -> Result<Vec<Attendance>, DomainError>;
    fn find_today(&self, today: &str) -> Result<Vec<Attendance>, DomainError>;
}
