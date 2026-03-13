use dioxus::prelude::*;
use std::sync::Arc;

use followup::application::use_cases::AttendanceService;
use followup::infrastructure::db::init_db;
use followup::infrastructure::sqlite_repository::SqliteAttendanceRepository;
use followup::presentation::routes::Route;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize DB and service once via use_hook (runs only on first render)
    use_context_provider(|| {
        let pool = init_db();
        let repo = SqliteAttendanceRepository::new(pool);
        Arc::new(AttendanceService::new(Box::new(repo)))
    });

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}
