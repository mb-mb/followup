use dioxus::prelude::*;
use std::sync::Arc;
use crate::application::use_cases::AttendanceService;
use crate::presentation::components::{StatusBadge, PriorityBadge, EmptyState};
use crate::presentation::routes::Route;

#[component]
pub fn Agenda() -> Element {
    let service = use_context::<Arc<AttendanceService>>();

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let svc1 = service.clone();
    let overdue = use_memo(move || {
        svc1.get_overdue().unwrap_or_default()
    });

    let svc2 = service.clone();
    let today_items = use_memo(move || {
        svc2.get_today().unwrap_or_default()
    });

    let svc3 = service.clone();
    let upcoming = use_memo(move || {
        svc3.get_upcoming().unwrap_or_default()
    });

    let today_val = today.clone();
    let grouped_upcoming = use_memo(move || {
        let items = upcoming();
        let mut groups: Vec<(String, Vec<crate::domain::entities::Attendance>)> = Vec::new();
        for item in items {
            if item.proximo_atendimento == today_val {
                continue;
            }
            if let Some(group) = groups.iter_mut().find(|(d, _)| *d == item.proximo_atendimento) {
                group.1.push(item);
            } else {
                groups.push((item.proximo_atendimento.clone(), vec![item]));
            }
        }
        groups
    });

    let has_any = !overdue().is_empty() || !today_items().is_empty() || !grouped_upcoming().is_empty();

    rsx! {
        div {
            class: "page agenda",
            h1 { class: "page-title", "📅 Agenda de Follow-ups" }
            p { class: "page-subtitle", "Próximos atendimentos agendados" }

            if !has_any {
                EmptyState {
                    icon: "📅".to_string(),
                    message: "Nenhum follow-up agendado no momento.".to_string(),
                }
            }

            if !overdue().is_empty() {
                div {
                    class: "agenda-section",
                    h2 { class: "agenda-date agenda-overdue", "⚠️ Atrasados" }
                    div {
                        class: "agenda-items",
                        for item in overdue() {
                            Link {
                                to: Route::AttendanceDetail { id: item.id.clone() },
                                class: "agenda-item item-overdue",
                                div {
                                    class: "agenda-time",
                                    span { class: "agenda-day", "{item.proximo_atendimento}" }
                                }
                                div {
                                    class: "agenda-info",
                                    div { class: "agenda-patient", "{item.paciente}" }
                                    div { class: "agenda-type", "{item.tipo_atendimento}" }
                                    div {
                                        class: "agenda-meta",
                                        span { "{item.canal_atendimento}" }
                                        span { " · " }
                                        span { "{item.profissional_responsavel}" }
                                    }
                                }
                                div {
                                    class: "agenda-badges",
                                    PriorityBadge { priority: item.prioridade.clone() }
                                }
                            }
                        }
                    }
                }
            }

            if !today_items().is_empty() {
                div {
                    class: "agenda-section",
                    h2 { class: "agenda-date agenda-today-title", "📅 Hoje" }
                    div {
                        class: "agenda-items",
                        for item in today_items() {
                            Link {
                                to: Route::AttendanceDetail { id: item.id.clone() },
                                class: "agenda-item item-today",
                                div {
                                    class: "agenda-info",
                                    div { class: "agenda-patient", "{item.paciente}" }
                                    div { class: "agenda-type", "{item.tipo_atendimento}" }
                                    div {
                                        class: "agenda-meta",
                                        span { "{item.canal_atendimento}" }
                                        span { " · " }
                                        span { "{item.profissional_responsavel}" }
                                    }
                                }
                                div {
                                    class: "agenda-badges",
                                    StatusBadge { status: item.status.clone() }
                                    PriorityBadge { priority: item.prioridade.clone() }
                                }
                            }
                        }
                    }
                }
            }

            for (date, items) in grouped_upcoming() {
                div {
                    class: "agenda-section",
                    h2 { class: "agenda-date", "📅 {date}" }
                    div {
                        class: "agenda-items",
                        for item in items {
                            Link {
                                to: Route::AttendanceDetail { id: item.id.clone() },
                                class: "agenda-item",
                                div {
                                    class: "agenda-info",
                                    div { class: "agenda-patient", "{item.paciente}" }
                                    div { class: "agenda-type", "{item.tipo_atendimento}" }
                                    div {
                                        class: "agenda-meta",
                                        span { "{item.canal_atendimento}" }
                                        span { " · " }
                                        span { "{item.profissional_responsavel}" }
                                    }
                                }
                                div {
                                    class: "agenda-badges",
                                    StatusBadge { status: item.status.clone() }
                                    PriorityBadge { priority: item.prioridade.clone() }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
