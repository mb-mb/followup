use dioxus::prelude::*;
use std::sync::Arc;
use crate::application::dto::UpdateAttendanceDto;
use crate::application::use_cases::AttendanceService;
use crate::domain::enums::{AttendanceChannel, AttendanceStatus, Priority};
use crate::presentation::components::{StatusBadge, PriorityBadge, ConfirmDialog};
use crate::presentation::routes::Route;

#[component]
pub fn AttendanceDetail(id: String) -> Element {
    let service = use_context::<Arc<AttendanceService>>();
    let nav = use_navigator();

    let mut editing = use_signal(|| false);
    let mut refresh_counter = use_signal(|| 0u32);
    let mut delete_confirm = use_signal(|| false);
    let mut toast_msg = use_signal(|| Option::<String>::None);
    let mut errors = use_signal(|| Vec::<String>::new());

    let id_clone = id.clone();
    let svc1 = service.clone();
    let attendance = use_memo(move || {
        refresh_counter();
        svc1.get_attendance(&id_clone).ok()
    });

    let mut paciente = use_signal(|| String::new());
    let mut tipo_atendimento = use_signal(|| String::new());
    let mut data_atendimento = use_signal(|| String::new());
    let mut proximo_atendimento = use_signal(|| String::new());
    let mut observacao = use_signal(|| String::new());
    let mut profissional = use_signal(|| String::new());
    let mut canal = use_signal(|| String::new());
    let mut prioridade = use_signal(|| String::new());
    let mut status = use_signal(|| String::new());

    let svc_delete = service.clone();
    let svc_update = service.clone();

    if let Some(ref a) = attendance() {
        if !editing() {
            let a_clone = a.clone();
            rsx! {
                div {
                    class: "page",
                    h1 { class: "page-title", "📋 Detalhe do Atendimento" }

                    if let Some(msg) = toast_msg() {
                        div { class: "toast toast-success", onclick: move |_| toast_msg.set(None), "{msg}" }
                    }

                    div {
                        class: "detail-card",
                        div {
                            class: "detail-header",
                            h2 { "{a.paciente}" }
                            div {
                                class: "card-badges",
                                StatusBadge { status: a.status.clone() }
                                PriorityBadge { priority: a.prioridade.clone() }
                            }
                        }

                        div {
                            class: "detail-grid",
                            div {
                                class: "detail-item",
                                span { class: "detail-label", "Tipo de Atendimento" }
                                span { class: "detail-value", "{a.tipo_atendimento}" }
                            }
                            div {
                                class: "detail-item",
                                span { class: "detail-label", "Data do Atendimento" }
                                span { class: "detail-value", "{a.data_atendimento}" }
                            }
                            div {
                                class: "detail-item",
                                span { class: "detail-label", "Próximo Atendimento" }
                                span { class: "detail-value", "{a.proximo_atendimento}" }
                            }
                            div {
                                class: "detail-item",
                                span { class: "detail-label", "Profissional" }
                                span { class: "detail-value", "{a.profissional_responsavel}" }
                            }
                            div {
                                class: "detail-item",
                                span { class: "detail-label", "Canal" }
                                span { class: "detail-value", "{a.canal_atendimento}" }
                            }
                            div {
                                class: "detail-item",
                                span { class: "detail-label", "Criado em" }
                                span { class: "detail-value", "{a.criado_em}" }
                            }
                            div {
                                class: "detail-item",
                                span { class: "detail-label", "Atualizado em" }
                                span { class: "detail-value", "{a.atualizado_em}" }
                            }
                        }

                        if !a.observacao.is_empty() {
                            div {
                                class: "detail-obs",
                                span { class: "detail-label", "Observações" }
                                p { class: "detail-obs-text", "{a.observacao}" }
                            }
                        }

                        div {
                            class: "detail-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    paciente.set(a_clone.paciente.clone());
                                    tipo_atendimento.set(a_clone.tipo_atendimento.clone());
                                    data_atendimento.set(a_clone.data_atendimento.clone());
                                    proximo_atendimento.set(a_clone.proximo_atendimento.clone());
                                    observacao.set(a_clone.observacao.clone());
                                    profissional.set(a_clone.profissional_responsavel.clone());
                                    canal.set(a_clone.canal_atendimento.to_string());
                                    prioridade.set(a_clone.prioridade.to_string());
                                    status.set(a_clone.status.to_string());
                                    editing.set(true);
                                },
                                "✏️ Editar"
                            }
                            button {
                                class: "btn btn-danger",
                                onclick: move |_| delete_confirm.set(true),
                                "🗑️ Excluir"
                            }
                            Link {
                                to: Route::AttendanceList {},
                                class: "btn btn-ghost",
                                "← Voltar"
                            }
                        }
                    }

                    if delete_confirm() {
                        { let id_del = id.clone();
                          rsx! {
                            ConfirmDialog {
                                message: "Tem certeza que deseja excluir este atendimento?".to_string(),
                                on_confirm: move |_| {
                                    let _ = svc_delete.delete_attendance(&id_del);
                                    nav.push(Route::AttendanceList {});
                                },
                                on_cancel: move |_| delete_confirm.set(false),
                            }
                          }
                        }
                    }
                }
            }
        } else {
            // Edit mode
            let id_upd = id.clone();
            rsx! {
                div {
                    class: "page",
                    h1 { class: "page-title", "✏️ Editar Atendimento" }

                    if !errors().is_empty() {
                        div {
                            class: "error-box",
                            for err in errors() {
                                p { "⚠️ {err}" }
                            }
                        }
                    }

                    form {
                        class: "form",
                        onsubmit: move |e| {
                            e.prevent_default();
                            let dto = UpdateAttendanceDto {
                                id: id_upd.clone(),
                                paciente: paciente(),
                                tipo_atendimento: tipo_atendimento(),
                                data_atendimento: data_atendimento(),
                                proximo_atendimento: proximo_atendimento(),
                                observacao: observacao(),
                                status: AttendanceStatus::from_str_opt(&status()).unwrap_or(AttendanceStatus::Pendente),
                                profissional_responsavel: profissional(),
                                canal_atendimento: AttendanceChannel::from_str_opt(&canal()).unwrap_or(AttendanceChannel::Presencial),
                                prioridade: Priority::from_str_opt(&prioridade()).unwrap_or(Priority::Media),
                            };
                            match svc_update.update_attendance(dto) {
                                Ok(_) => {
                                    editing.set(false);
                                    errors.set(Vec::new());
                                    toast_msg.set(Some("Atendimento atualizado com sucesso!".to_string()));
                                    refresh_counter.set(refresh_counter() + 1);
                                }
                                Err(e) => {
                                    errors.set(vec![e.to_string()]);
                                }
                            }
                        },

                        div {
                            class: "form-group",
                            label { "Paciente *" }
                            input {
                                class: "input",
                                r#type: "text",
                                value: "{paciente}",
                                oninput: move |e| paciente.set(e.value()),
                            }
                        }

                        div {
                            class: "form-group",
                            label { "Tipo de Atendimento *" }
                            input {
                                class: "input",
                                r#type: "text",
                                value: "{tipo_atendimento}",
                                oninput: move |e| tipo_atendimento.set(e.value()),
                            }
                        }

                        div {
                            class: "form-row",
                            div {
                                class: "form-group",
                                label { "Data do Atendimento *" }
                                input {
                                    class: "input",
                                    r#type: "date",
                                    value: "{data_atendimento}",
                                    oninput: move |e| data_atendimento.set(e.value()),
                                }
                            }
                            div {
                                class: "form-group",
                                label { "Próximo Atendimento *" }
                                input {
                                    class: "input",
                                    r#type: "date",
                                    value: "{proximo_atendimento}",
                                    oninput: move |e| proximo_atendimento.set(e.value()),
                                }
                            }
                        }

                        div {
                            class: "form-group",
                            label { "Profissional Responsável *" }
                            input {
                                class: "input",
                                r#type: "text",
                                value: "{profissional}",
                                oninput: move |e| profissional.set(e.value()),
                            }
                        }

                        div {
                            class: "form-row",
                            div {
                                class: "form-group",
                                label { "Status" }
                                select {
                                    class: "input",
                                    value: "{status}",
                                    onchange: move |e| status.set(e.value()),
                                    for s in AttendanceStatus::all() {
                                        option { value: "{s}", "{s}" }
                                    }
                                }
                            }
                            div {
                                class: "form-group",
                                label { "Canal" }
                                select {
                                    class: "input",
                                    value: "{canal}",
                                    onchange: move |e| canal.set(e.value()),
                                    for c in AttendanceChannel::all() {
                                        option { value: "{c}", "{c}" }
                                    }
                                }
                            }
                            div {
                                class: "form-group",
                                label { "Prioridade" }
                                select {
                                    class: "input",
                                    value: "{prioridade}",
                                    onchange: move |e| prioridade.set(e.value()),
                                    for p in Priority::all() {
                                        option { value: "{p}", "{p}" }
                                    }
                                }
                            }
                        }

                        div {
                            class: "form-group",
                            label { "Observações" }
                            textarea {
                                class: "input textarea",
                                rows: "4",
                                value: "{observacao}",
                                oninput: move |e| observacao.set(e.value()),
                            }
                        }

                        div {
                            class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                r#type: "submit",
                                "💾 Salvar"
                            }
                            button {
                                class: "btn btn-ghost",
                                r#type: "button",
                                onclick: move |_| {
                                    editing.set(false);
                                    errors.set(Vec::new());
                                },
                                "Cancelar"
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "page",
                div {
                    class: "empty-state",
                    span { class: "empty-icon", "❌" }
                    p { "Atendimento não encontrado." }
                    Link {
                        to: Route::AttendanceList {},
                        class: "btn btn-primary",
                        "← Voltar para lista"
                    }
                }
            }
        }
    }
}
