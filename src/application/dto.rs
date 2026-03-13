use crate::domain::enums::{AttendanceChannel, AttendanceStatus, Priority};

#[derive(Debug, Clone, PartialEq)]
pub struct CreateAttendanceDto {
    pub paciente: String,
    pub tipo_atendimento: String,
    pub data_atendimento: String,
    pub proximo_atendimento: String,
    pub observacao: String,
    pub profissional_responsavel: String,
    pub canal_atendimento: AttendanceChannel,
    pub prioridade: Priority,
}

impl CreateAttendanceDto {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.paciente.trim().is_empty() {
            errors.push("Paciente é obrigatório".to_string());
        }
        if self.tipo_atendimento.trim().is_empty() {
            errors.push("Tipo de atendimento é obrigatório".to_string());
        }
        if self.data_atendimento.trim().is_empty() {
            errors.push("Data do atendimento é obrigatória".to_string());
        }
        if self.proximo_atendimento.trim().is_empty() {
            errors.push("Próximo atendimento é obrigatório".to_string());
        }
        if self.profissional_responsavel.trim().is_empty() {
            errors.push("Profissional responsável é obrigatório".to_string());
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateAttendanceDto {
    pub id: String,
    pub paciente: String,
    pub tipo_atendimento: String,
    pub data_atendimento: String,
    pub proximo_atendimento: String,
    pub observacao: String,
    pub status: AttendanceStatus,
    pub profissional_responsavel: String,
    pub canal_atendimento: AttendanceChannel,
    pub prioridade: Priority,
}

impl UpdateAttendanceDto {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.paciente.trim().is_empty() {
            errors.push("Paciente é obrigatório".to_string());
        }
        if self.tipo_atendimento.trim().is_empty() {
            errors.push("Tipo de atendimento é obrigatório".to_string());
        }
        if self.data_atendimento.trim().is_empty() {
            errors.push("Data do atendimento é obrigatória".to_string());
        }
        if self.proximo_atendimento.trim().is_empty() {
            errors.push("Próximo atendimento é obrigatório".to_string());
        }
        if self.profissional_responsavel.trim().is_empty() {
            errors.push("Profissional responsável é obrigatório".to_string());
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AttendanceFilter {
    pub search: String,
    pub status: Option<AttendanceStatus>,
    pub prioridade: Option<Priority>,
    pub canal: Option<AttendanceChannel>,
}

impl AttendanceFilter {
    pub fn matches(&self, a: &crate::domain::entities::Attendance) -> bool {
        if !self.search.is_empty() {
            let s = self.search.to_lowercase();
            let matches_search = a.paciente.to_lowercase().contains(&s)
                || a.tipo_atendimento.to_lowercase().contains(&s)
                || a.profissional_responsavel.to_lowercase().contains(&s)
                || a.observacao.to_lowercase().contains(&s);
            if !matches_search {
                return false;
            }
        }
        if let Some(ref status) = self.status {
            if &a.status != status {
                return false;
            }
        }
        if let Some(ref prio) = self.prioridade {
            if &a.prioridade != prio {
                return false;
            }
        }
        if let Some(ref canal) = self.canal {
            if &a.canal_atendimento != canal {
                return false;
            }
        }
        true
    }
}
