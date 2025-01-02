use std::time::{Instant, Duration};

use eframe::egui::*;
use yahoo_finance_api::YMetaData;

use crate::yahoo_api_helper::{YahooFetchHandle, fetch_now_data};

struct StockInfo {
	ticker: String,
	price: f64,
	fetch_handle: YahooFetchHandle,
	metadata: Option<YMetaData>,
}

impl StockInfo {
	pub fn new(ticker: &str) -> Self {
		Self {
			ticker: ticker.to_string(),
			price: 0.,
			fetch_handle: None,
			metadata: None,
		}
	}
}

pub struct StockSidePanel {
	stock_list: Vec<StockInfo>,
	timer: Instant
}

impl StockSidePanel {
	pub fn new() -> Self {
		let ticker_list = vec![StockInfo::new("TSLA"),
							   StockInfo::new("GOOGL"),
							   StockInfo::new("AMZN"),
							   StockInfo::new("NVDA"),
							   StockInfo::new("AMD"),
							   StockInfo::new("INTC"),
							   StockInfo::new("MSFT"),
							   StockInfo::new("META"),
							   StockInfo::new("NFLX"),
							   StockInfo::new("PLTR"),
							   StockInfo::new("MCD"),
							   StockInfo::new("KO"),
							   StockInfo::new("MA"),
							   StockInfo::new("SPOT"),
							   StockInfo::new("AAPL")];

		Self {
			stock_list: ticker_list,
			timer: Instant::now()
		}
	}

	pub fn show(&mut self, ui: &mut Ui, change_ticker: &mut Option<String>) {
		if self.timer.elapsed() > Duration::from_secs(2) {
			for stock in &mut self.stock_list {
				fetch_now_data(&mut stock.fetch_handle, &stock.ticker, &mut stock.price, &mut stock.metadata);
			}
			self.timer = Instant::now();
		}

		ScrollArea::vertical().show(ui, |ui| {
			for stock in &self.stock_list {
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
		                        Label::new(
		                            RichText::new(stock.ticker.clone())
		                                .heading().strong(),
		                        ).selectable(false).ui(ui);

			                    let latest_price = stock.price;
			                    let mut start_price = None;
							
								let currency = match &stock.metadata {
							        Some(metadata) => {
										start_price = metadata.previous_close;
										metadata.currency.clone().unwrap()
									},
							        None => "".to_string()
							    };
								ui.label(format!("{:.2} {}", stock.price, currency));

								if let Some(start_price) = start_price {
				                    let p_change = (latest_price - start_price) / start_price * 100.;

									ui.horizontal(|ui| {
				                        if p_change > 0. {
				                            ui.label(RichText::new(format!("+{:.2}%", p_change)).color(Color32::GREEN));
				                            ui.label(RichText::new(format!("+{:.2} {}", start_price * (p_change / 100.), currency)).color(Color32::GREEN));
				                        } else if p_change < 0. {
				                            ui.label(RichText::new(format!("{:.2}%", p_change)).color(Color32::RED));
				                            ui.label(RichText::new(format!("{:.2} {}", start_price * (p_change / 100.), currency)).color(Color32::RED));
				                        } else {
				                            ui.label(format!("+{:.2}%", p_change));
				                        }
									});
								}
		                    });
		            }).response;

		        if response.clicked() {
					*change_ticker = Some(stock.ticker.to_string());
		        }
			}
		});
	}
}
