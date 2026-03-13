use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AttendanceStatus {
    Pendente,
    Realizado,
    Atrasado,
    Cancelado,
}

impl AttendanceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pendente => "Pendente",
            Self::Realizado => "Realizado",
            Self::Atrasado => "Atrasado",
            Self::Cancelado => "Cancelado",
        }
    }

    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "Pendente" => Some(Self::Pendente),
            "Realizado" => Some(Self::Realizado),
            "Atrasado" => Some(Self::Atrasado),
            "Cancelado" => Some(Self::Cancelado),
            _ => None,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Pendente, Self::Realizado, Self::Atrasado, Self::Cancelado]
    }
}

impl fmt::Display for AttendanceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AttendanceChannel {
    Presencial,
    Telefone,
    WhatsApp,
    Video,
}

impl AttendanceChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Presencial => "Presencial",
            Self::Telefone => "Telefone",
            Self::WhatsApp => "WhatsApp",
            Self::Video => "Vídeo",
        }
    }

    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "Presencial" => Some(Self::Presencial),
            "Telefone" => Some(Self::Telefone),
            "WhatsApp" => Some(Self::WhatsApp),
            "Vídeo" | "Video" => Some(Self::Video),
            _ => None,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Presencial, Self::Telefone, Self::WhatsApp, Self::Video]
    }
}

impl fmt::Display for AttendanceChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Priority {
    Baixa,
    Media,
    Alta,
    Urgente,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Baixa => "Baixa",
            Self::Media => "Média",
            Self::Alta => "Alta",
            Self::Urgente => "Urgente",
        }
    }

    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "Baixa" => Some(Self::Baixa),
            "Média" | "Media" => Some(Self::Media),
            "Alta" => Some(Self::Alta),
            "Urgente" => Some(Self::Urgente),
            _ => None,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Baixa, Self::Media, Self::Alta, Self::Urgente]
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
