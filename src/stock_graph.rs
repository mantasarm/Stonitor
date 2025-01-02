use std::{time::{Duration, UNIX_EPOCH}, ops::RangeInclusive};

use eframe::egui::*;
use egui_plot::*;
use yahoo_finance_api::{time::OffsetDateTime, YMetaData};

use crate::yahoo_api_helper::{YahooFetchHandle, fetch_recent_interval, fetch_history};

pub struct StockGraph {
    ticker: String,
    pub price_data: Vec<[f64; 2]>,
	pub volume_data: Vec<[f64; 2]>,
	reset_plot: bool,
    fetch_handle: YahooFetchHandle,
	pub metadata: Option<YMetaData>,
	pub data_range: String
}

impl StockGraph {
	pub fn new(ticker: &str) -> Self {
		Self {
			ticker: ticker.to_string(),
			price_data: vec![],
			volume_data: vec![],
			reset_plot: false,
		    fetch_handle: None,
			metadata: None,
			data_range: "Regular".to_string()
		}
	}

	pub fn show(&mut self, ui: &mut Ui) {
        // Axis formatting
        let time_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| {
            let timestamp = mark.value;

            let time = OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(timestamp as u64));
            
            format!("{:02}:{:02}:{:02}", time.hour(), time.minute(), time.second())
        };

        let x_hint = AxisHints::new_x().formatter(time_formatter);
        let y_hint_price = AxisHints::new_y();

        // Cursor label formatter
        let label_fmt = |_s: &str, val: &PlotPoint| {
            let time = OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(val.x as u64));
            format!("Time: {:02}:{:02}:{:02}\nPrice: {:.4}", time.hour(), time.minute(), time.second(), val.y)
        };

		let link_group_id = ui.id().with("linked_demo");
		
        let mut my_plot = Plot::new(self.ticker.clone())
							.link_axis(link_group_id, true)
                            .legend(Legend::default())
                            .custom_x_axes(vec![x_hint.clone()])
                            .custom_y_axes(vec![y_hint_price])
                            .label_formatter(label_fmt)
							.allow_scroll(false);

		if self.reset_plot {
			my_plot = my_plot.reset();
			self.reset_plot = false;
		}

		let (mut min_price, mut max_price, mut min_volume, mut max_volume) = (0., 0., 0., 0.);
		if !self.price_data.is_empty() && !self.volume_data.is_empty() {
			min_price = self.price_data[0][1];
			max_price = self.price_data[0][1];
			for price in &self.price_data {
				if price[1] < min_price {
					min_price = price[1];
				}
				if price[1] > max_price {
					max_price = price[1];
				}
			}
	
			min_volume = self.volume_data[0][1];
			max_volume = self.volume_data[0][1];
			for volume in &self.volume_data {
				if volume[1] < min_volume{
					min_volume = volume[1];
				}
				if volume[1] > max_volume{
					max_volume = volume[1];
				}
			}
		}

		// Price chart
        my_plot.show(ui, |plot_ui| {
			let mut bars = vec![];
			for volume in &self.volume_data {
				bars.push(Bar::new(volume[0], (map_value(volume[1], min_volume, max_volume, min_price, max_price) - min_price) / 10.).base_offset(min_price));
			}
			plot_ui.bar_chart(BarChart::new(bars).color(Color32::LIGHT_BLUE).width(20.).allow_hover(false).name("Volume"));
			
            plot_ui.line(Line::new(PlotPoints::from(self.price_data.clone())).name(&self.ticker));
			// if !self.price_data.is_empty() {
			// 	plot_ui.vline(VLine::new(self.price_data[0][0]).name("Market open"));
			// }
        });

		ui.horizontal(|ui| {
			ui.label("Range:");
			if ui.button("Regular").clicked() {
				self.data_range = "Regular".to_string();
				self.reset_plot = true;
			}
			if ui.button("1mo").clicked() {
				self.data_range = "1mo".to_string();
				self.reset_plot = true;
			}
			if ui.button("3mo").clicked() {
				self.data_range = "3mo".to_string();
				self.reset_plot = true;
			}
			if ui.button("6mo").clicked() {
				self.data_range = "6mo".to_string();
				self.reset_plot = true;
			}
			if ui.button("1y").clicked() {
				self.data_range = "1y".to_string();
				self.reset_plot = true;
			}
			if ui.button("ytd").clicked() {
				self.data_range = "ytd".to_string();
				self.reset_plot = true;
			}
			if ui.button("max").clicked() {
				self.data_range = "max".to_string();
				self.reset_plot = true;
			}
		});
	}

	pub fn change_ticker(&mut self, ticker: &str) {
		self.ticker = ticker.to_string();
		self.reset_plot = true;
	}

	pub fn update_data(&mut self) {
		if self.data_range == "Regular" {
			fetch_recent_interval(&mut self.fetch_handle, &self.ticker, &mut self.price_data, &mut self.volume_data, &mut self.metadata);
		} else {
			fetch_history(&mut self.fetch_handle, &self.ticker, &self.data_range, &mut self.price_data, &mut self.volume_data, &mut self.metadata);
		}
	}
}

pub fn map_value(value: f64, begin: f64, end: f64, new_begin: f64, new_end: f64) -> f64 {
    new_begin + (new_end - new_begin) * ((value - begin) / (end - begin))
}
