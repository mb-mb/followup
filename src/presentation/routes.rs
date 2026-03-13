use dioxus::prelude::*;
use crate::presentation::components::NavBar;
use crate::presentation::pages::{
    dashboard::Dashboard,
    attendance_list::AttendanceList,
    new_attendance::NewAttendance,
    attendance_detail::AttendanceDetail,
    agenda::Agenda,
};

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Dashboard {},
        #[route("/atendimentos")]
        AttendanceList {},
        #[route("/novo")]
        NewAttendance {},
        #[route("/atendimento/:id")]
        AttendanceDetail { id: String },
        #[route("/agenda")]
        Agenda {},
}

#[component]
fn AppLayout() -> Element {
    rsx! {
        NavBar {}
        main {
            class: "main-content",
            Outlet::<Route> {}
        }
    }
}
