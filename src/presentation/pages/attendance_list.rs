use dioxus::prelude::*;
use std::sync::Arc;
use crate::application::dto::AttendanceFilter;
use crate::application::use_cases::AttendanceService;
use crate::domain::enums::{AttendanceStatus, AttendanceChannel, Priority};
use crate::presentation::components::{StatusBadge, PriorityBadge, ConfirmDialog, EmptyState};
use crate::presentation::routes::Route;

#[component]
pub fn AttendanceList() -> Element {
    let service = use_context::<Arc<AttendanceService>>();
    let nav = use_navigator();

    let mut refresh_counter = use_signal(|| 0u32);
    let mut search = use_signal(|| String::new());
    let mut status_filter = use_signal(|| Option::<AttendanceStatus>::None);
    let mut priority_filter = use_signal(|| Option::<Priority>::None);
    let mut channel_filter = use_signal(|| Option::<AttendanceChannel>::None);
    let mut delete_target = use_signal(|| Option::<String>::None);
    let mut toast_msg = use_signal(|| Option::<String>::None);

    let svc1 = service.clone();
    let all_items = use_memo(move || {
        refresh_counter();
        svc1.list_attendances().unwrap_or_default()
    });

    let filtered = use_memo(move || {
        let filter = AttendanceFilter {
            search: search(),
            status: status_filter(),
            prioridade: priority_filter(),
            canal: channel_filter(),
        };
        all_items().into_iter().filter(|a| filter.matches(a)).collect::<Vec<_>>()
    });

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let svc_delete = service.clone();

    rsx! {
        div {
            class: "page",
            h1 { class: "page-title", "📋 Atendimentos" }

            div {
                class: "filters",
                div {
                    class: "filter-row",
                    input {
                        class: "input search-input",
                        r#type: "text",
                        placeholder: "🔍 Buscar paciente, tipo ou profissional...",
                        value: "{search}",
                        oninput: move |e| search.set(e.value()),
                    }
                }
                div {
                    class: "filter-row",
                    select {
                        class: "input select-input",
                        onchange: move |e| {
                            status_filter.set(AttendanceStatus::from_str_opt(&e.value()));
                        },
                        option { value: "", "Todos os status" }
                        for s in AttendanceStatus::all() {
                            option { value: "{s}", "{s}" }
                        }
                    }
                    select {
                        class: "input select-input",
                        onchange: move |e| {
                            priority_filter.set(Priority::from_str_opt(&e.value()));
                        },
                        option { value: "", "Todas as prioridades" }
                        for p in Priority::all() {
                            option { value: "{p}", "{p}" }
                        }
                    }
                    select {
                        class: "input select-input",
                        onchange: move |e| {
                            channel_filter.set(AttendanceChannel::from_str_opt(&e.value()));
                        },
                        option { value: "", "Todos os canais" }
                        for c in AttendanceChannel::all() {
                            option { value: "{c}", "{c}" }
                        }
                    }
                }
            }

            p {
                class: "results-count",
                "{filtered().len()} atendimento(s) encontrado(s)"
            }

            if filtered().is_empty() {
                EmptyState {
                    icon: "🔍".to_string(),
                    message: "Nenhum atendimento encontrado com os filtros aplicados.".to_string(),
                }
            } else {
                div {
                    class: "table-container",
                    table {
                        class: "data-table",
                        thead {
                            tr {
                                th { "Paciente" }
                                th { "Tipo" }
                                th { "Próximo" }
                                th { "Status" }
                                th { "Prioridade" }
                                th { "Canal" }
                                th { "Ações" }
                            }
                        }
                        tbody {
                            for item in filtered() {
                                { let is_overdue = item.status == AttendanceStatus::Pendente && item.proximo_atendimento < today;
                                  let is_today_item = item.proximo_atendimento == today;
                                  let row_class = if is_overdue { "row-overdue" } else if is_today_item { "row-today" } else { "" };
                                  let item_id = item.id.clone();
                                  let item_id2 = item.id.clone();
                                  rsx! {
                                    tr {
                                        class: "{row_class}",
                                        td { class: "td-patient", "{item.paciente}" }
                                        td { "{item.tipo_atendimento}" }
                                        td { class: if is_overdue { "td-overdue" } else { "" },
                                            "{item.proximo_atendimento}"
                                        }
                                        td { StatusBadge { status: item.status.clone() } }
                                        td { PriorityBadge { priority: item.prioridade.clone() } }
                                        td { "{item.canal_atendimento}" }
                                        td {
                                            class: "td-actions",
                                            button {
                                                class: "btn btn-sm btn-primary",
                                                onclick: move |_| {
                                                    nav.push(Route::AttendanceDetail { id: item_id.clone() });
                                                },
                                                "Editar"
                                            }
                                            button {
                                                class: "btn btn-sm btn-danger",
                                                onclick: move |_| delete_target.set(Some(item_id2.clone())),
                                                "Excluir"
                                            }
                                        }
                                    }
                                  }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(target_id) = delete_target() {
                ConfirmDialog {
                    message: "Tem certeza que deseja excluir este atendimento?".to_string(),
                    on_confirm: move |_| {
                        let _ = svc_delete.delete_attendance(&target_id);
                        delete_target.set(None);
                        toast_msg.set(Some("Atendimento excluído com sucesso!".to_string()));
                        refresh_counter.set(refresh_counter() + 1);
                    },
                    on_cancel: move |_| delete_target.set(None),
                }
            }

            if let Some(msg) = toast_msg() {
                div {
                    class: "toast toast-success",
                    onclick: move |_| toast_msg.set(None),
                    "{msg}"
                }
            }
        }
    }
}
