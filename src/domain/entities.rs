use crate::domain::enums::{AttendanceChannel, AttendanceStatus, Priority};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attendance {
    pub id: String,
    pub paciente: String,
    pub tipo_atendimento: String,
    pub data_atendimento: String,       // YYYY-MM-DD
    pub proximo_atendimento: String,    // YYYY-MM-DD
    pub observacao: String,
    pub status: AttendanceStatus,
    pub profissional_responsavel: String,
    pub canal_atendimento: AttendanceChannel,
    pub prioridade: Priority,
    pub criado_em: String,              // YYYY-MM-DD HH:MM:SS
    pub atualizado_em: String,          // YYYY-MM-DD HH:MM:SS
}

impl Attendance {
    pub fn is_overdue(&self, today: &str) -> bool {
        self.status == AttendanceStatus::Pendente && self.proximo_atendimento < today.to_string()
    }

    pub fn is_today(&self, today: &str) -> bool {
        self.proximo_atendimento == today
    }
}
