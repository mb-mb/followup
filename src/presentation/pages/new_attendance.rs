use dioxus::prelude::*;
use std::sync::Arc;
use crate::application::dto::CreateAttendanceDto;
use crate::application::use_cases::AttendanceService;
use crate::domain::enums::{AttendanceChannel, Priority};
use crate::presentation::routes::Route;

#[component]
pub fn NewAttendance() -> Element {
    let service = use_context::<Arc<AttendanceService>>();
    let nav = use_navigator();

    let mut paciente = use_signal(|| String::new());
    let mut tipo_atendimento = use_signal(|| String::new());
    let mut data_atendimento = use_signal(|| String::new());
    let mut proximo_atendimento = use_signal(|| String::new());
    let mut observacao = use_signal(|| String::new());
    let mut profissional = use_signal(|| String::new());
    let mut canal = use_signal(|| "Presencial".to_string());
    let mut prioridade = use_signal(|| "Média".to_string());
    let mut errors = use_signal(|| Vec::<String>::new());
    let mut success_msg = use_signal(|| Option::<String>::None);

    let svc_save = service.clone();
    let svc_save2 = service.clone();

    rsx! {
        div {
            class: "page",
            h1 { class: "page-title", "➕ Novo Atendimento" }

            if let Some(msg) = success_msg() {
                div { class: "toast toast-success", "{msg}" }
            }

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
                    let dto = CreateAttendanceDto {
                        paciente: paciente(),
                        tipo_atendimento: tipo_atendimento(),
                        data_atendimento: data_atendimento(),
                        proximo_atendimento: proximo_atendimento(),
                        observacao: observacao(),
                        profissional_responsavel: profissional(),
                        canal_atendimento: AttendanceChannel::from_str_opt(&canal()).unwrap_or(AttendanceChannel::Presencial),
                        prioridade: Priority::from_str_opt(&prioridade()).unwrap_or(Priority::Media),
                    };
                    match svc_save.create_attendance(dto) {
                        Ok(_) => {
                            nav.push(Route::AttendanceList {});
                        }
                        Err(e) => {
                            errors.set(vec![e.to_string()]);
                            success_msg.set(None);
                        }
                    }
                },

                div {
                    class: "form-group",
                    label { r#for: "paciente", "Paciente *" }
                    input {
                        id: "paciente",
                        class: "input",
                        r#type: "text",
                        placeholder: "Nome do paciente",
                        value: "{paciente}",
                        oninput: move |e| paciente.set(e.value()),
                    }
                }

                div {
                    class: "form-group",
                    label { r#for: "tipo", "Tipo de Atendimento *" }
                    input {
                        id: "tipo",
                        class: "input",
                        r#type: "text",
                        placeholder: "Ex: Consulta, Curativo, Vacinação...",
                        value: "{tipo_atendimento}",
                        oninput: move |e| tipo_atendimento.set(e.value()),
                    }
                }

                div {
                    class: "form-row",
                    div {
                        class: "form-group",
                        label { r#for: "data", "Data do Atendimento *" }
                        input {
                            id: "data",
                            class: "input",
                            r#type: "date",
                            value: "{data_atendimento}",
                            oninput: move |e| data_atendimento.set(e.value()),
                        }
                    }
                    div {
                        class: "form-group",
                        label { r#for: "proximo", "Próximo Atendimento *" }
                        input {
                            id: "proximo",
                            class: "input",
                            r#type: "date",
                            value: "{proximo_atendimento}",
                            oninput: move |e| proximo_atendimento.set(e.value()),
                        }
                    }
                }

                div {
                    class: "form-group",
                    label { r#for: "profissional", "Profissional Responsável *" }
                    input {
                        id: "profissional",
                        class: "input",
                        r#type: "text",
                        placeholder: "Nome do(a) enfermeiro(a)",
                        value: "{profissional}",
                        oninput: move |e| profissional.set(e.value()),
                    }
                }

                div {
                    class: "form-row",
                    div {
                        class: "form-group",
                        label { r#for: "canal", "Canal" }
                        select {
                            id: "canal",
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
                        label { r#for: "prioridade", "Prioridade" }
                        select {
                            id: "prioridade",
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
                    label { r#for: "obs", "Observações" }
                    textarea {
                        id: "obs",
                        class: "input textarea",
                        placeholder: "Observações sobre o atendimento...",
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
                        class: "btn btn-secondary",
                        r#type: "button",
                        onclick: move |_| {
                            let dto = CreateAttendanceDto {
                                paciente: paciente(),
                                tipo_atendimento: tipo_atendimento(),
                                data_atendimento: data_atendimento(),
                                proximo_atendimento: proximo_atendimento(),
                                observacao: observacao(),
                                profissional_responsavel: profissional(),
                                canal_atendimento: AttendanceChannel::from_str_opt(&canal()).unwrap_or(AttendanceChannel::Presencial),
                                prioridade: Priority::from_str_opt(&prioridade()).unwrap_or(Priority::Media),
                            };
                            match svc_save2.create_attendance(dto) {
                                Ok(_) => {
                                    paciente.set(String::new());
                                    tipo_atendimento.set(String::new());
                                    data_atendimento.set(String::new());
                                    proximo_atendimento.set(String::new());
                                    observacao.set(String::new());
                                    profissional.set(String::new());
                                    canal.set("Presencial".to_string());
                                    prioridade.set("Média".to_string());
                                    errors.set(Vec::new());
                                    success_msg.set(Some("Atendimento salvo! Preencha um novo.".to_string()));
                                }
                                Err(e) => {
                                    errors.set(vec![e.to_string()]);
                                    success_msg.set(None);
                                }
                            }
                        },
                        "💾 Salvar e Novo"
                    }
                    Link {
                        to: Route::AttendanceList {},
                        class: "btn btn-ghost",
                        "Cancelar"
                    }
                }
            }
        }
    }
}
