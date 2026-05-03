use rust_i18n::t;

use crate::ui::app::App;

pub fn render_search_form(app: &mut App, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    const MENU_HEIGHT: f32 = 100.0;
    const SEARCH_FORM_SIDE_MARGIN: f32 = 350.0;
    const SEARCH_FORM_HEIGHT: f32 = 800.0; // Random value, only used space will be allocated.
    const WIDGET_ALL_MARGIN: f32 = 20.0;
    const WIDGETS_PER_ROW: f32 = 3.0;

    let SEARCH_FORM_WIDTH: f32 = app.state.window_rect.width() - (SEARCH_FORM_SIDE_MARGIN * 2.0);
    let WIDGET_WIDTH: f32 =
        (SEARCH_FORM_WIDTH - ((WIDGETS_PER_ROW + 1.0) * WIDGET_ALL_MARGIN)) / WIDGETS_PER_ROW;
    let WIDGET_HEIGHT: f32 = 20.0;

    let SEARCH_FORM_TOP_LEFT =
        app.state.window_rect.left_top() + egui::vec2(SEARCH_FORM_SIDE_MARGIN, MENU_HEIGHT);
    let SEARCH_FORM_BOTTOM_RIGHT = app.state.window_rect.right_bottom()
        - egui::vec2(SEARCH_FORM_SIDE_MARGIN, SEARCH_FORM_HEIGHT);

    let search_form_rec = egui::Rect::from_two_pos(SEARCH_FORM_TOP_LEFT, SEARCH_FORM_BOTTOM_RIGHT);

    // A B C
    // D E F
    // G H I
    // ...
    ui.allocate_ui_at_rect(search_form_rec, |ui| {
        ui.group(|ui| {
            let A_top_left =
                SEARCH_FORM_TOP_LEFT + egui::vec2(WIDGET_ALL_MARGIN, WIDGET_ALL_MARGIN);
            let A_bottom_right = A_top_left + egui::vec2(WIDGET_WIDTH, WIDGET_HEIGHT);
            let A = egui::Rect::from_two_pos(A_top_left, A_bottom_right);

            let B_top_left = A.right_top() + egui::vec2(WIDGET_ALL_MARGIN, 0.0);
            let B_bottom_right = B_top_left + egui::vec2(WIDGET_WIDTH, WIDGET_HEIGHT);
            let B = egui::Rect::from_two_pos(B_top_left, B_bottom_right);

            let C_top_left = B.right_top() + egui::vec2(WIDGET_ALL_MARGIN, 0.0);
            let C_bottom_right = C_top_left + egui::vec2(WIDGET_WIDTH, WIDGET_HEIGHT);
            let C = egui::Rect::from_two_pos(C_top_left, C_bottom_right);

            let D_top_left = A.left_bottom() + egui::vec2(0.0, WIDGET_ALL_MARGIN);
            let D_bottom_right = D_top_left + egui::vec2(WIDGET_WIDTH, WIDGET_HEIGHT);
            let D = egui::Rect::from_two_pos(D_top_left, D_bottom_right);

            let E_top_left = D.right_top() + egui::vec2(WIDGET_ALL_MARGIN, 0.0);
            let E_bottom_right = E_top_left + egui::vec2(WIDGET_WIDTH, WIDGET_HEIGHT);
            let E = egui::Rect::from_two_pos(E_top_left, E_bottom_right);

            let F_top_left = E.right_top() + egui::vec2(WIDGET_ALL_MARGIN, 0.0);
            let F_bottom_right = F_top_left + egui::vec2(WIDGET_WIDTH, WIDGET_HEIGHT);
            let F = egui::Rect::from_two_pos(F_top_left, F_bottom_right);

            ui.allocate_ui_at_rect(A, |ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut app.search_part_of_name)
                        .hint_text(t!("search_input_part_of_name")),
                );
            });
            ui.allocate_ui_at_rect(B, |ui| {
                app.search_name_widget.ui(ui);
            });
            ui.allocate_ui_at_rect(C, |ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut app.search_barecode)
                        .hint_text(t!("search_input_barecode")),
                );
            });
            ui.allocate_ui_at_rect(D, |ui| {
                app.search_cas_number_widget.ui(ui);
            });
            ui.allocate_ui_at_rect(E, |ui| {
                app.search_empirical_formula_widget.ui(ui);
            });
            ui.allocate_ui_at_rect(F, |ui| {
                app.search_producer_ref_widget.ui(ui);
            });

            ui.collapsing(t!("advanced_search"), |ui| {
                app.state.advanced_search_rect = ui.max_rect();

                let ADVANCED_SEARCH_FORM_WIDTH: f32 = app.state.advanced_search_rect.width();
                let ADVANCED_SEARCH_FORM_TOP_LEFT = app.state.advanced_search_rect.left_top();
                let ADVANCED_SEARCH_WIDGET_WIDTH: f32 = (ADVANCED_SEARCH_FORM_WIDTH
                    - ((WIDGETS_PER_ROW + 1.0) * WIDGET_ALL_MARGIN))
                    / WIDGETS_PER_ROW;
                let ADVANCED_SEARCH_WIDGET_HEIGHT: f32 = 20.0;

                let G_top_left = ADVANCED_SEARCH_FORM_TOP_LEFT
                    + egui::vec2(WIDGET_ALL_MARGIN, WIDGET_ALL_MARGIN);
                let G_bottom_right = G_top_left
                    + egui::vec2(ADVANCED_SEARCH_WIDGET_WIDTH, ADVANCED_SEARCH_WIDGET_HEIGHT);
                let G = egui::Rect::from_two_pos(G_top_left, G_bottom_right);

                ui.allocate_ui_at_rect(G, |ui| {
                    app.search_entity_widget.ui(ui);
                });
            });
        });
    });
}
