// Entry point. Real modules are declared here and wired up in `app`.
mod actions;
mod app;
mod host;
mod i18n;
mod ignore_store;
mod scan_service;
mod updater;
mod ui {
    pub mod ignored_tab;
    pub mod port_row;
    pub mod project_group;
    pub mod tray;
    pub mod window;
}

fn main() {
    app::run();
}
