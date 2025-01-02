use eframe::egui::*;
use yahoo_finance_api::YSearchResult;

use crate::{yahoo_api_helper::{YahooFetchSearchHandle, fetch_search_ticker}, stock_graph::StockGraph};

pub struct SearchBar {
    search_text: String,
	prev_search_text: String,
    search_handle: YahooFetchSearchHandle,
    search_result: Option<YSearchResult>,
	pub searching: bool,
	found_result: bool
}

impl SearchBar {
	pub fn new() -> Self {
		Self {
            search_text: "".to_string(),
			prev_search_text: "".to_string(),
			search_handle: None,
			search_result: None,
			searching: false,
			found_result: false
		}
	}

	pub fn show(&mut self, ui: &mut Ui, stock_graph: &mut StockGraph) {
        let text_edit = TextEdit::singleline(&mut self.search_text).hint_text("Enter stock name");
		let response = ui.add(text_edit);
		
        ui.separator();
		
        if response.gained_focus() {
			self.searching = true;
		}

		if self.searching {
			if self.prev_search_text != self.search_text {
				self.found_result = false;
			}

			if !self.found_result {
		        fetch_search_ticker(&mut self.search_handle, &self.search_text, &mut self.search_result, &mut self.found_result);
			}

	        if let Some(s_result) = &self.search_result {
	            for result in &s_result.quotes {
					let qtype = result.quote_type.as_str();
					if matches!(qtype, "MUTUALFUND" | "INDEX" | "OPTION" | "CURRENCY" | "FUTURE") {
						continue;
					}
						
					let response = ui.scope_builder(
						UiBuilder::new().sense(Sense::click()), |ui| {
						    let response = ui.response();
						    let visuals = ui.style().interact(&response);

						    Frame::canvas(ui.style())
						        .fill(visuals.bg_fill.gamma_multiply(0.3))
						        .stroke(visuals.bg_stroke)
						        .inner_margin(ui.spacing().menu_margin)
						        .show(ui, |ui| {
						            ui.set_width(ui.available_width());

									let name_label = Label::new(RichText::new(format!("{}", result.short_name)).heading().strong()).selectable(false);

									ui.add(name_label);
									ui.horizontal(|ui| {
										ui.label(format!("{}", result.symbol));
										ui.label(format!("{}", result.exchange));
										ui.label(format!("{}", result.quote_type));
									});
						        });
					}).response;

					if response.clicked() {
						stock_graph.change_ticker(&result.symbol);
					}
	            }
	        }
		}

		if response.lost_focus() && self.search_text.is_empty() {
			self.searching = false;
		}

		self.prev_search_text = self.search_text.clone();
	}
}
