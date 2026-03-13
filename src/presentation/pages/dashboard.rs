use dioxus::prelude::*;
use std::sync::Arc;
use crate::application::use_cases::AttendanceService;
use crate::presentation::components::{StatusBadge, PriorityBadge, EmptyState};
use crate::presentation::routes::Route;

#[component]
pub fn Dashboard() -> Element {
    let service = use_context::<Arc<AttendanceService>>();

    let svc1 = service.clone();
    let all = use_memo(move || {
        svc1.list_attendances().unwrap_or_default()
    });

    let svc2 = service.clone();
    let overdue = use_memo(move || {
        svc2.get_overdue().unwrap_or_default()
    });

    let svc3 = service.clone();
    let today_items = use_memo(move || {
        svc3.get_today().unwrap_or_default()
    });

    let total = use_memo(move || all().len());
    let pending_count = use_memo(move || {
        all().iter().filter(|a| a.status == crate::domain::enums::AttendanceStatus::Pendente).count()
    });
    let done_count = use_memo(move || {
        all().iter().filter(|a| a.status == crate::domain::enums::AttendanceStatus::Realizado).count()
    });
    let overdue_count = use_memo(move || overdue().len());
    let today_count = use_memo(move || today_items().len());

    rsx! {
        div {
            class: "page dashboard",
            h1 { class: "page-title", "📊 Dashboard" }
            p { class: "page-subtitle", "Visão geral dos atendimentos" }

            div {
                class: "stats-grid",
                div {
                    class: "stat-card",
                    div { class: "stat-icon", "📋" }
                    div { class: "stat-value", "{total}" }
                    div { class: "stat-label", "Total" }
                }
                div {
                    class: "stat-card stat-today",
                    div { class: "stat-icon", "📅" }
                    div { class: "stat-value", "{today_count}" }
                    div { class: "stat-label", "Hoje" }
                }
                div {
                    class: "stat-card stat-overdue",
                    div { class: "stat-icon", "⚠️" }
                    div { class: "stat-value", "{overdue_count}" }
                    div { class: "stat-label", "Atrasados" }
                }
                div {
                    class: "stat-card stat-pending",
                    div { class: "stat-icon", "⏳" }
                    div { class: "stat-value", "{pending_count}" }
                    div { class: "stat-label", "Pendentes" }
                }
                div {
                    class: "stat-card stat-done",
                    div { class: "stat-icon", "✅" }
                    div { class: "stat-value", "{done_count}" }
                    div { class: "stat-label", "Realizados" }
                }
            }

            if !overdue().is_empty() {
                div {
                    class: "section",
                    h2 { class: "section-title section-overdue", "⚠️ Follow-ups Atrasados" }
                    div {
                        class: "card-list",
                        for item in overdue() {
                            Link {
                                to: Route::AttendanceDetail { id: item.id.clone() },
                                class: "attendance-card card-overdue",
                                div {
                                    class: "card-header",
                                    span { class: "card-patient", "{item.paciente}" }
                                    PriorityBadge { priority: item.prioridade.clone() }
                                }
                                div {
                                    class: "card-body",
                                    span { class: "card-type", "{item.tipo_atendimento}" }
                                    span { class: "card-date overdue-date", "Vencido: {item.proximo_atendimento}" }
                                }
                            }
                        }
                    }
                }
            }

            if !today_items().is_empty() {
                div {
                    class: "section",
                    h2 { class: "section-title section-today", "📅 Atendimentos de Hoje" }
                    div {
                        class: "card-list",
                        for item in today_items() {
                            Link {
                                to: Route::AttendanceDetail { id: item.id.clone() },
                                class: "attendance-card card-today",
                                div {
                                    class: "card-header",
                                    span { class: "card-patient", "{item.paciente}" }
                                    StatusBadge { status: item.status.clone() }
                                }
                                div {
                                    class: "card-body",
                                    span { class: "card-type", "{item.tipo_atendimento}" }
                                    span { class: "card-channel", "{item.canal_atendimento}" }
                                }
                            }
                        }
                    }
                }
            }

            div {
                class: "section",
                h2 { class: "section-title", "📋 Atendimentos Recentes" }
                if all().is_empty() {
                    EmptyState {
                        icon: "📋".to_string(),
                        message: "Nenhum atendimento registrado ainda.".to_string(),
                    }
                } else {
                    div {
                        class: "card-list",
                        for item in all().into_iter().take(5) {
                            Link {
                                to: Route::AttendanceDetail { id: item.id.clone() },
                                class: "attendance-card",
                                div {
                                    class: "card-header",
                                    span { class: "card-patient", "{item.paciente}" }
                                    div {
                                        class: "card-badges",
                                        StatusBadge { status: item.status.clone() }
                                        PriorityBadge { priority: item.prioridade.clone() }
                                    }
                                }
                                div {
                                    class: "card-body",
                                    span { class: "card-type", "{item.tipo_atendimento}" }
                                    span { class: "card-date", "Próximo: {item.proximo_atendimento}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
