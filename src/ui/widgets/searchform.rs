use crate::ui::app::App;

pub fn render_search_form(app: &mut App, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    app.search_store_location_widget.ui(ui);
    app.search_name_widget.ui(ui);
}
