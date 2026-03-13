use crate::domain::entities::Attendance;
use crate::domain::enums::{AttendanceChannel, AttendanceStatus, Priority};
use crate::domain::errors::DomainError;
use crate::domain::repositories::AttendanceRepository;
use crate::infrastructure::db::DbPool;
use rusqlite::params;

pub struct SqliteAttendanceRepository {
    pool: DbPool,
}

impl SqliteAttendanceRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn row_to_attendance(row: &rusqlite::Row) -> rusqlite::Result<Attendance> {
        let status_str: String = row.get(6)?;
        let canal_str: String = row.get(8)?;
        let prio_str: String = row.get(9)?;

        Ok(Attendance {
            id: row.get(0)?,
            paciente: row.get(1)?,
            tipo_atendimento: row.get(2)?,
            data_atendimento: row.get(3)?,
            proximo_atendimento: row.get(4)?,
            observacao: row.get(5)?,
            status: AttendanceStatus::from_str_opt(&status_str).unwrap_or(AttendanceStatus::Pendente),
            profissional_responsavel: row.get(7)?,
            canal_atendimento: AttendanceChannel::from_str_opt(&canal_str).unwrap_or(AttendanceChannel::Presencial),
            prioridade: Priority::from_str_opt(&prio_str).unwrap_or(Priority::Media),
            criado_em: row.get(10)?,
            atualizado_em: row.get(11)?,
        })
    }
}

impl AttendanceRepository for SqliteAttendanceRepository {
    fn create(&self, attendance: &Attendance) -> Result<(), DomainError> {
        let conn = self.pool.lock().map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        conn.execute(
            "INSERT INTO attendances (id, paciente, tipo_atendimento, data_atendimento, proximo_atendimento, observacao, status, profissional_responsavel, canal_atendimento, prioridade, criado_em, atualizado_em)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                attendance.id,
                attendance.paciente,
                attendance.tipo_atendimento,
                attendance.data_atendimento,
                attendance.proximo_atendimento,
                attendance.observacao,
                attendance.status.as_str(),
                attendance.profissional_responsavel,
                attendance.canal_atendimento.as_str(),
                attendance.prioridade.as_str(),
                attendance.criado_em,
                attendance.atualizado_em,
            ],
        ).map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    fn update(&self, attendance: &Attendance) -> Result<(), DomainError> {
        let conn = self.pool.lock().map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        let rows = conn.execute(
            "UPDATE attendances SET paciente=?1, tipo_atendimento=?2, data_atendimento=?3, proximo_atendimento=?4, observacao=?5, status=?6, profissional_responsavel=?7, canal_atendimento=?8, prioridade=?9, atualizado_em=?10 WHERE id=?11",
            params![
                attendance.paciente,
                attendance.tipo_atendimento,
                attendance.data_atendimento,
                attendance.proximo_atendimento,
                attendance.observacao,
                attendance.status.as_str(),
                attendance.profissional_responsavel,
                attendance.canal_atendimento.as_str(),
                attendance.prioridade.as_str(),
                attendance.atualizado_em,
                attendance.id,
            ],
        ).map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    fn delete(&self, id: &str) -> Result<(), DomainError> {
        let conn = self.pool.lock().map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        let rows = conn.execute("DELETE FROM attendances WHERE id=?1", params![id])
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        if rows == 0 {
            return Err(DomainError::NotFound);
        }
        Ok(())
    }

    fn find_by_id(&self, id: &str) -> Result<Attendance, DomainError> {
        let conn = self.pool.lock().map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        conn.query_row(
            "SELECT id, paciente, tipo_atendimento, data_atendimento, proximo_atendimento, observacao, status, profissional_responsavel, canal_atendimento, prioridade, criado_em, atualizado_em FROM attendances WHERE id=?1",
            params![id],
            Self::row_to_attendance,
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DomainError::NotFound,
            _ => DomainError::DatabaseError(e.to_string()),
        })
    }

    fn find_all(&self) -> Result<Vec<Attendance>, DomainError> {
        let conn = self.pool.lock().map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, paciente, tipo_atendimento, data_atendimento, proximo_atendimento, observacao, status, profissional_responsavel, canal_atendimento, prioridade, criado_em, atualizado_em FROM attendances ORDER BY proximo_atendimento ASC"
        ).map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        let rows = stmt.query_map([], Self::row_to_attendance)
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| DomainError::DatabaseError(e.to_string()))?);
        }
        Ok(result)
    }

    fn find_by_date_range(&self, start: &str, end: &str) -> Result<Vec<Attendance>, DomainError> {
        let conn = self.pool.lock().map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, paciente, tipo_atendimento, data_atendimento, proximo_atendimento, observacao, status, profissional_responsavel, canal_atendimento, prioridade, criado_em, atualizado_em FROM attendances WHERE proximo_atendimento >= ?1 AND proximo_atendimento <= ?2 AND status = 'Pendente' ORDER BY proximo_atendimento ASC"
        ).map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        let rows = stmt.query_map(params![start, end], Self::row_to_attendance)
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| DomainError::DatabaseError(e.to_string()))?);
        }
        Ok(result)
    }

    fn find_overdue(&self, today: &str) -> Result<Vec<Attendance>, DomainError> {
        let conn = self.pool.lock().map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, paciente, tipo_atendimento, data_atendimento, proximo_atendimento, observacao, status, profissional_responsavel, canal_atendimento, prioridade, criado_em, atualizado_em FROM attendances WHERE proximo_atendimento < ?1 AND status = 'Pendente' ORDER BY proximo_atendimento ASC"
        ).map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        let rows = stmt.query_map(params![today], Self::row_to_attendance)
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| DomainError::DatabaseError(e.to_string()))?);
        }
        Ok(result)
    }

    fn find_today(&self, today: &str) -> Result<Vec<Attendance>, DomainError> {
        let conn = self.pool.lock().map_err(|e| DomainError::DatabaseError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, paciente, tipo_atendimento, data_atendimento, proximo_atendimento, observacao, status, profissional_responsavel, canal_atendimento, prioridade, criado_em, atualizado_em FROM attendances WHERE proximo_atendimento = ?1 ORDER BY prioridade DESC"
        ).map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        let rows = stmt.query_map(params![today], Self::row_to_attendance)
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| DomainError::DatabaseError(e.to_string()))?);
        }
        Ok(result)
    }
}
