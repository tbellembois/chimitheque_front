use crate::ui::app::App;
use crate::ui::state::Action;
use crate::ui::widgets::size::Size;
use crate::ui::widgets::{
    buttonwithiconandtext::button_with_icon_and_text,
    clickablelabelwithiconandtext::clickable_label_with_icon_and_text,
};
use crate::utils::base64_to_egui_texture;
use egui::{RichText, TextBuffer};
use rust_i18n::t;

const PRODUCT_LABEL_INNER_MARGIN: egui::Margin = egui::Margin::symmetric(20, 10);
const PRODUCT_LABEL_CORNER_RADIUS: f32 = 8.0;

pub fn update(app: &mut App, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    let widgets = &ui.visuals().widgets;
    let stroke = widgets.noninteractive.bg_stroke;

    // egui's ui.group does not support margins, so we use a custom frame instead.
    let custom_group_frame = egui::Frame::new()
        .inner_margin(PRODUCT_LABEL_INNER_MARGIN)
        .corner_radius(PRODUCT_LABEL_CORNER_RADIUS)
        .stroke(egui::Stroke::new(1.0, stroke.color));

    ui.vertical_centered(|ui| {
        ui.set_width(300.0);

        ui.horizontal(|ui| {
            let response = ui.add(
                egui::TextEdit::singleline(&mut app.pubchem_search)
                    .hint_text(t!("pubchem_input_search")),
            );

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                app.state.action = Action::GetPubchemAutocomplete;
            }

            ui.add_space(20.0);

            if button_with_icon_and_text(
                ui,
                t!("pubchem_action_search").to_string(),
                egui_phosphor::fill::MAGNIFYING_GLASS,
                &Size::Small,
            )
            .clicked()
            {
                app.pubchem_results_expanded = true;
                app.state.action = Action::GetPubchemAutocomplete;
            }
        });

        ui.add_space(20.0);

        if app.pubchem_results_expanded {
            if let Ok(may_be_pubchem_autocomplete) = app.get_pubchem_autocomplete()
                && let Some(pubchem_autocomplete) = may_be_pubchem_autocomplete
                && let Some(dictionary_terms) = pubchem_autocomplete.dictionary_terms
            {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    for compound in dictionary_terms.compound {
                        if ui.link(compound.clone()).clicked() {
                            app.pubchem_search_name_clicked = compound;
                            app.pubchem_results_expanded = false;
                            app.state.action = Action::GetPubchemProduct;
                        }
                    }
                });
            }
        } else if let Ok(may_be_pubchem_autocomplete) = app.get_pubchem_autocomplete()
            && may_be_pubchem_autocomplete.is_some()
            && clickable_label_with_icon_and_text(
                ui,
                t!("search_pubchem_results_expand").as_str(),
                egui_phosphor::regular::MAGNIFYING_GLASS,
                &Size::Small,
            )
            .clicked()
        {
            app.pubchem_results_expanded = true;
        }

        ui.add_space(20.0);

        if let Ok(may_be_pubchem_product) = app.get_pubchem_product()
            && let Some(pubchem_product) = may_be_pubchem_product
        {
            egui::ScrollArea::vertical()
                .id_salt("pubchem_product_scrollarea")
                .show(ui, |ui| {
                    custom_group_frame.show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.style_mut().spacing.item_spacing.y = 15.0;

                            if let Some(twodpicture) = pubchem_product.twodpicture {
                                let ctx = ui.ctx();
                                let texture = base64_to_egui_texture(
                                    ctx,
                                    twodpicture.as_str(),
                                    "twodpicture",
                                );
                                ui.image(&texture.unwrap());
                            }

                            if let Some(name) = pubchem_product.name {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(t!("product_card_name")).italics());
                                    ui.label(name);
                                });
                            }

                            if let Some(iupac_name) = pubchem_product.iupac_name {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(t!("product_card_iupac_name")).italics(),
                                    );
                                    ui.label(iupac_name);
                                });
                            }

                            if let Some(molecular_formula) = pubchem_product.molecular_formula {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(t!("product_card_empirical_formula"))
                                            .italics(),
                                    );
                                    ui.label(molecular_formula);
                                });
                            }

                            if let Some(cas) = pubchem_product.cas {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(t!("product_card_cas_number")).italics(),
                                    );
                                    ui.label(cas);
                                });
                            }

                            if let Some(ec) = pubchem_product.ec {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(t!("product_card_ce_number")).italics());
                                    ui.label(ec);
                                });
                            }

                            if let Some(synonyms) = pubchem_product.synonyms {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(t!("product_card_synonyms")).italics());
                                    ui.vertical(|ui| {
                                        for synonym in synonyms {
                                            ui.label(synonym);
                                        }
                                    });
                                });
                            }
                        });
                    });
                });
        }
    });
}
