use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub type DbPool = Arc<Mutex<Connection>>;

pub fn init_db() -> DbPool {
    let conn = Connection::open("followup.db")
        .expect("Falha ao abrir banco de dados SQLite");

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS attendances (
            id TEXT PRIMARY KEY,
            paciente TEXT NOT NULL,
            tipo_atendimento TEXT NOT NULL,
            data_atendimento TEXT NOT NULL,
            proximo_atendimento TEXT NOT NULL,
            observacao TEXT NOT NULL DEFAULT '',
            status TEXT NOT NULL DEFAULT 'Pendente',
            profissional_responsavel TEXT NOT NULL,
            canal_atendimento TEXT NOT NULL DEFAULT 'Presencial',
            prioridade TEXT NOT NULL DEFAULT 'Media',
            criado_em TEXT NOT NULL,
            atualizado_em TEXT NOT NULL
        );"
    ).expect("Falha ao criar tabela attendances");

    let pool = Arc::new(Mutex::new(conn));
    seed_sample_data(&pool);
    pool
}

fn seed_sample_data(pool: &DbPool) {
    let conn = pool.lock().unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM attendances", [], |row| row.get(0))
        .unwrap_or(0);

    if count > 0 {
        return;
    }

    let now = chrono::Local::now();
    let today = now.format("%Y-%m-%d").to_string();
    let yesterday = (now - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
    let two_days_ago = (now - chrono::Duration::days(2)).format("%Y-%m-%d").to_string();
    let three_days_ago = (now - chrono::Duration::days(3)).format("%Y-%m-%d").to_string();
    let tomorrow = (now + chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
    let in_3_days = (now + chrono::Duration::days(3)).format("%Y-%m-%d").to_string();
    let in_7_days = (now + chrono::Duration::days(7)).format("%Y-%m-%d").to_string();
    let in_14_days = (now + chrono::Duration::days(14)).format("%Y-%m-%d").to_string();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let samples = vec![
        ("Maria da Silva", "Consulta de rotina", &three_days_ago, &today, "Paciente estável, manter medicação", "Pendente", "Enf. Ana Costa", "Presencial", "Media"),
        ("João Oliveira", "Curativo", &two_days_ago, &yesterday, "Troca de curativo na perna esquerda", "Pendente", "Enf. Ana Costa", "Presencial", "Alta"),
        ("Ana Souza", "Acompanhamento pós-cirúrgico", &yesterday, &tomorrow, "Recuperação dentro do esperado", "Pendente", "Enf. Carlos Lima", "Telefone", "Alta"),
        ("Carlos Mendes", "Orientação medicamentosa", &today, &in_3_days, "Ajuste de dosagem de insulina", "Pendente", "Enf. Ana Costa", "WhatsApp", "Urgente"),
        ("Lucia Ferreira", "Vacinação", &two_days_ago, &in_7_days, "Aplicação de vacina da gripe, retorno para reforço", "Realizado", "Enf. Beatriz Santos", "Presencial", "Baixa"),
        ("Pedro Alves", "Revisão de exames", &three_days_ago, &in_14_days, "Resultados de hemograma dentro da normalidade", "Realizado", "Enf. Carlos Lima", "Vídeo", "Media"),
        ("Fernanda Costa", "Curativo", &yesterday, &today, "Curativo infectado, necessita reavaliação urgente", "Pendente", "Enf. Beatriz Santos", "Presencial", "Urgente"),
        ("Roberto Santos", "Acompanhamento crônico", &three_days_ago, &two_days_ago, "Hipertensão descontrolada", "Pendente", "Enf. Ana Costa", "Telefone", "Alta"),
    ];

    for (paciente, tipo, data, proximo, obs, status, prof, canal, prio) in samples {
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO attendances (id, paciente, tipo_atendimento, data_atendimento, proximo_atendimento, observacao, status, profissional_responsavel, canal_atendimento, prioridade, criado_em, atualizado_em)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            rusqlite::params![id, paciente, tipo, data, proximo, obs, status, prof, canal, prio, timestamp, timestamp],
        ).expect("Falha ao inserir dados de exemplo");
    }
}
