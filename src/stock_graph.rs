use std::{time::{Duration, UNIX_EPOCH}, ops::RangeInclusive};

use eframe::egui::*;
use egui_plot::*;
use yahoo_finance_api::{time::OffsetDateTime, YMetaData};

use crate::yahoo_api_helper::{YahooFetchHandle, fetch_period_interval};

pub struct StockGraph {
    ticker: String,
    pub price_data: Vec<[f64; 2]>,
	pub volume_data: Vec<[f64; 2]>,
	reset_plot: bool,
    fetch_handle: YahooFetchHandle,
	pub metadata: Option<YMetaData>
}

impl StockGraph {
	pub fn new(ticker: &str) -> Self {
		Self {
			ticker: ticker.to_string(),
			price_data: vec![],
			volume_data: vec![],
			reset_plot: false,
		    fetch_handle: None,
			metadata: None
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
            format!("Time: {:02}:{:02}:{:02}\nPrice: {}", time.hour(), time.minute(), time.second(), val.y)
        };

		let link_group_id = ui.id().with("linked_demo");
		
        let mut my_plot = Plot::new(self.ticker.clone())
							.link_axis(link_group_id, true, false)
                            .legend(Legend::default())
                            .custom_x_axes(vec![x_hint.clone()])
                            .custom_y_axes(vec![y_hint_price])
                            .label_formatter(label_fmt)
							.height(900.)
							.allow_scroll(false);

		if self.reset_plot {
			my_plot = my_plot.reset();
			self.reset_plot = false;
		}

		// Price chart
        my_plot.show(ui, |plot_ui| {
            plot_ui.line(Line::new(PlotPoints::from(self.price_data.clone())).name(&self.ticker));

			if !self.price_data.is_empty() {
				plot_ui.vline(VLine::new(self.price_data[0][0]).name("Market open"));
				plot_ui.vline(VLine::new(self.price_data[self.price_data.len() - 1][0]).name("Market close"));
			}
			
        });


		// Volume bar chart
        let formatter_empty = |_: GridMark, _: &RangeInclusive<f64>| {
			"".to_string()
        };
        let x_hint = AxisHints::new_x().formatter(formatter_empty);
		let y_hint = AxisHints::new_y().formatter(formatter_empty);
        let label_fmt = |_s: &str, val: &PlotPoint| {
            let time = OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(val.x as u64));
            format!("Time: {:02}:{:02}:{:02}\nVolume: {}", time.hour(), time.minute(), time.second(), val.y as u64)
        };
		
		Plot::new("volume")
			.link_axis(link_group_id, true, false)
			.custom_x_axes(vec![x_hint])
			.custom_y_axes(vec![y_hint])
			.label_formatter(label_fmt)
			.allow_scroll(false)
			.allow_boxed_zoom(false)
			.allow_drag(false)
			.show(ui, |plot_ui| {
			let mut bars = vec![];
			for volume in &self.volume_data {
				bars.push(Bar::new(volume[0], volume[1]))
			}

			plot_ui.bar_chart(BarChart::new(bars).color(Color32::LIGHT_BLUE).width(20.));

		});
	}

	pub fn change_ticker(&mut self, ticker: &str) {
		self.ticker = ticker.to_string();
		self.reset_plot = true;
	}

	pub fn update_data(&mut self) {
		fetch_period_interval(&mut self.fetch_handle, &self.ticker, &mut self.price_data, &mut self.volume_data, &mut self.metadata);
	}
}
