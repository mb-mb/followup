use dioxus::prelude::*;
use crate::presentation::routes::Route;
use crate::domain::enums::{AttendanceStatus, Priority};

#[component]
pub fn NavBar() -> Element {
    let route: Route = use_route();

    let is_active = |r: &Route| -> &'static str {
        if std::mem::discriminant(&route) == std::mem::discriminant(r) {
            "nav-link active"
        } else {
            "nav-link"
        }
    };

    rsx! {
        nav {
            class: "navbar",
            div {
                class: "nav-brand",
                Link {
                    to: Route::Dashboard {},
                    class: "brand-link",
                    span { class: "brand-icon", "🏥" }
                    span { class: "brand-text", "FollowUp" }
                }
            }
            div {
                class: "nav-links",
                Link {
                    to: Route::Dashboard {},
                    class: is_active(&Route::Dashboard {}),
                    "📊 Dashboard"
                }
                Link {
                    to: Route::AttendanceList {},
                    class: is_active(&Route::AttendanceList {}),
                    "📋 Atendimentos"
                }
                Link {
                    to: Route::NewAttendance {},
                    class: is_active(&Route::NewAttendance {}),
                    "➕ Novo"
                }
                Link {
                    to: Route::Agenda {},
                    class: is_active(&Route::Agenda {}),
                    "📅 Agenda"
                }
            }
        }
    }
}

#[component]
pub fn StatusBadge(status: AttendanceStatus) -> Element {
    let class = match status {
        AttendanceStatus::Pendente => "badge badge-pending",
        AttendanceStatus::Realizado => "badge badge-done",
        AttendanceStatus::Atrasado => "badge badge-overdue",
        AttendanceStatus::Cancelado => "badge badge-cancelled",
    };
    rsx! {
        span { class: "{class}", "{status}" }
    }
}

#[component]
pub fn PriorityBadge(priority: Priority) -> Element {
    let class = match priority {
        Priority::Baixa => "badge badge-low",
        Priority::Media => "badge badge-medium",
        Priority::Alta => "badge badge-high",
        Priority::Urgente => "badge badge-urgent",
    };
    rsx! {
        span { class: "{class}", "{priority}" }
    }
}

#[component]
pub fn Toast(message: String, toast_type: String) -> Element {
    let class = format!("toast toast-{toast_type}");
    rsx! {
        div {
            class: "{class}",
            "{message}"
        }
    }
}

#[component]
pub fn ConfirmDialog(
    message: String,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "modal-overlay",
            onclick: move |_| on_cancel.call(()),
            div {
                class: "modal-content",
                onclick: move |e| e.stop_propagation(),
                p { class: "modal-message", "{message}" }
                div {
                    class: "modal-actions",
                    button {
                        class: "btn btn-danger",
                        onclick: move |_| on_confirm.call(()),
                        "Confirmar"
                    }
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| on_cancel.call(()),
                        "Cancelar"
                    }
                }
            }
        }
    }
}

#[component]
pub fn EmptyState(icon: String, message: String) -> Element {
    rsx! {
        div {
            class: "empty-state",
            span { class: "empty-icon", "{icon}" }
            p { "{message}" }
        }
    }
}
