use crate::application::dto::{CreateAttendanceDto, UpdateAttendanceDto};
use crate::domain::entities::Attendance;
use crate::domain::enums::AttendanceStatus;
use crate::domain::errors::DomainError;
use crate::domain::repositories::AttendanceRepository;

pub struct AttendanceService {
    repo: Box<dyn AttendanceRepository>,
}

impl AttendanceService {
    pub fn new(repo: Box<dyn AttendanceRepository>) -> Self {
        Self { repo }
    }

    pub fn create_attendance(&self, dto: CreateAttendanceDto) -> Result<Attendance, DomainError> {
        dto.validate().map_err(|errs| {
            DomainError::ValidationError(errs.join("; "))
        })?;

        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let attendance = Attendance {
            id: uuid::Uuid::new_v4().to_string(),
            paciente: dto.paciente,
            tipo_atendimento: dto.tipo_atendimento,
            data_atendimento: dto.data_atendimento,
            proximo_atendimento: dto.proximo_atendimento,
            observacao: dto.observacao,
            status: AttendanceStatus::Pendente,
            profissional_responsavel: dto.profissional_responsavel,
            canal_atendimento: dto.canal_atendimento,
            prioridade: dto.prioridade,
            criado_em: now.clone(),
            atualizado_em: now,
        };

        self.repo.create(&attendance)?;
        Ok(attendance)
    }

    pub fn update_attendance(&self, dto: UpdateAttendanceDto) -> Result<Attendance, DomainError> {
        dto.validate().map_err(|errs| {
            DomainError::ValidationError(errs.join("; "))
        })?;

        let existing = self.repo.find_by_id(&dto.id)?;
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let updated = Attendance {
            id: existing.id,
            paciente: dto.paciente,
            tipo_atendimento: dto.tipo_atendimento,
            data_atendimento: dto.data_atendimento,
            proximo_atendimento: dto.proximo_atendimento,
            observacao: dto.observacao,
            status: dto.status,
            profissional_responsavel: dto.profissional_responsavel,
            canal_atendimento: dto.canal_atendimento,
            prioridade: dto.prioridade,
            criado_em: existing.criado_em,
            atualizado_em: now,
        };

        self.repo.update(&updated)?;
        Ok(updated)
    }

    pub fn delete_attendance(&self, id: &str) -> Result<(), DomainError> {
        self.repo.delete(id)
    }

    pub fn get_attendance(&self, id: &str) -> Result<Attendance, DomainError> {
        self.repo.find_by_id(id)
    }

    pub fn list_attendances(&self) -> Result<Vec<Attendance>, DomainError> {
        self.repo.find_all()
    }

    pub fn get_overdue(&self) -> Result<Vec<Attendance>, DomainError> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.repo.find_overdue(&today)
    }

    pub fn get_today(&self) -> Result<Vec<Attendance>, DomainError> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.repo.find_today(&today)
    }

    pub fn get_upcoming(&self) -> Result<Vec<Attendance>, DomainError> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let future = chrono::Local::now()
            .checked_add_signed(chrono::Duration::days(30))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();
        self.repo.find_by_date_range(&today, &future)
    }
}
