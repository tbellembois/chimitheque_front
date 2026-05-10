use chimitheque_types::casnumber::CasNumber;
use chimitheque_types::empiricalformula::EmpiricalFormula;
use chimitheque_types::product::Product;
use egui_extras::{Column, TableBuilder};
use rust_i18n::t;

use crate::ui::app::App;
use crate::ui::widgets::buttonwithicon::button_with_icon;

const PRODUCT_LABEL_OUTER_MARGIN: egui::Margin = egui::Margin::symmetric(127, 5);
const PRODUCT_LABEL_INNER_MARGIN: egui::Margin = egui::Margin::symmetric(20, 10);
const PRODUCT_LABEL_MENU_WIDTH: f32 = 50.0;
const PRODUCT_LABEL_PLUS_WIDTH: f32 = 50.0;
const PRODUCT_LABEL_CORNER_RADIUS: f32 = 8.0;
const PAGE_RIGHT_MARGIN: f32 = 50.0;
const PAGE_LEFT_MARGIN: f32 = 50.0;

pub fn render_product_label(
    app: &mut App,
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
    product: Product,
) {
    let widgets = &ui.visuals().widgets;
    let stroke = widgets.noninteractive.bg_stroke;

    let label_available_width = app.state.window_rect.width()
        - PAGE_RIGHT_MARGIN
        - PAGE_LEFT_MARGIN
        - (PRODUCT_LABEL_OUTER_MARGIN.leftf() * 2.0)
        - (PRODUCT_LABEL_INNER_MARGIN.leftf() * 2.0);
    let cols_width =
        (label_available_width - PRODUCT_LABEL_MENU_WIDTH - PRODUCT_LABEL_PLUS_WIDTH) / 4.0;

    // egui's ui.group does not support margins, so we use a custom frame instead.
    let custom_group_frame = egui::Frame::new()
        .inner_margin(PRODUCT_LABEL_INNER_MARGIN)
        .outer_margin(PRODUCT_LABEL_OUTER_MARGIN)
        .corner_radius(PRODUCT_LABEL_CORNER_RADIUS)
        .stroke(egui::Stroke::new(1.0, stroke.color));

    custom_group_frame.show(ui, |ui| {
        TableBuilder::new(ui)
            .column(Column::exact(PRODUCT_LABEL_PLUS_WIDTH))
            .column(Column::exact(cols_width))
            .column(Column::exact(cols_width))
            .column(Column::exact(cols_width))
            .column(Column::exact(cols_width))
            .column(Column::exact(PRODUCT_LABEL_MENU_WIDTH))
            .id_salt(format!(
                "{}_product_label",
                product.product_id.unwrap_or_default()
            ))
            .body(|mut body| {
                body.row(50.0, |mut row| {
                    row.col(
                        |ui| {
                            if button_with_icon(ui, egui_phosphor::regular::PLUS).clicked() {}
                        },
                    );

                    row.col(|ui| {
                        ui.label(product.name.name_label);

                        ui.horizontal(|ui| {
                            ui.label(product.product_sl.unwrap_or_default());
                            if let Some(product_sc) = product.product_sc {
                                ui.label(format!(
                                    "[ {}: {product_sc} ]",
                                    t!("product_label_storages")
                                ));
                            }
                        });
                    });

                    row.col(|ui| {
                        ui.label(
                            product
                                .empirical_formula
                                .unwrap_or(EmpiricalFormula {
                                    empirical_formula_label: "".to_string(),
                                    ..Default::default()
                                })
                                .empirical_formula_label,
                        );
                    });

                    row.col(|ui| {
                        ui.label(
                            product
                                .cas_number
                                .clone()
                                .unwrap_or(CasNumber {
                                    cas_number_label: "".to_string(),
                                    ..Default::default()
                                })
                                .cas_number_label,
                        );
                    });

                    row.col(|ui| {
                        // Show CAS number CMR category.
                        if let Some(cas_number) = product.cas_number
                            && let Some(cas_number_cmr) = cas_number.cas_number_cmr
                        {
                            ui.label(cas_number_cmr);
                        }

                        // Show inflammable symbol.
                        ui.horizontal(|ui| {
                            if let Some(hs_cmr) = product.product_hs_cmr {
                                ui.label(hs_cmr);
                            };

                            if let Some(symbols) = product.symbols {
                                for symbol in symbols {
                                    if symbol.symbol_label == "GHS02".to_string() {
                                        ui.add(egui::Image::new(egui::include_image!(
                                            "../media/GHS02.svg"
                                        )));
                                    }
                                }
                            };
                        });
                    });

                    row.col(
                        |ui| {
                            if button_with_icon(ui, egui_phosphor::regular::LIST).clicked() {}
                        },
                    );
                });
            });
    });
}
