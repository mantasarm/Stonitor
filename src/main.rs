pub mod stock_graph;
pub mod yahoo_api_helper;
pub mod side_panel;
pub mod search_bar;

use eframe::{egui::{self, RichText}, epaint::Color32};
use search_bar::SearchBar;
use side_panel::StockSidePanel;
use stock_graph::StockGraph;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1200.0]),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Stonitor",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    stock_graph: StockGraph,
    stock_side_panel: StockSidePanel,
    search_bar: SearchBar
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            stock_graph: StockGraph::new("TSLA"),
            stock_side_panel: StockSidePanel::new(),
            search_bar: SearchBar::new()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.stock_graph.update_data();

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            self.search_bar.show(ui, &mut self.stock_graph);

            if !self.search_bar.searching {
                let mut change_ticker = None;
                self.stock_side_panel.show(ui, &mut change_ticker);
                if let Some(ticker) = change_ticker {
                    self.stock_graph.change_ticker(&ticker);
                }
            }
        });
        
        
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(metadata) = &self.stock_graph.metadata {
                    let latest_price = self.stock_graph.price_data[self.stock_graph.price_data.len() - 1][1];
                    let start_price = match metadata.previous_close {
                        Some(pc) => pc,
                        None => self.stock_graph.price_data[0][1]
                    };
                    let p_change = (latest_price - start_price) / start_price * 100.;

                    let currency = match metadata.currency.as_ref() {
                        Some(currency) => currency,
                        None => ""
                    };
                    
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(metadata.symbol.clone()).size(30.).strong());

                        if p_change > 0. {
                            ui.label(RichText::new(format!("+{:.4}%", p_change)).heading().color(Color32::GREEN));
                            ui.label(RichText::new(format!("+{:.4} {}", start_price * (p_change / 100.), currency)).heading().color(Color32::GREEN));
                        } else if p_change < 0. {
                            ui.label(RichText::new(format!("{:.4}%", p_change)).heading().color(Color32::RED));
                            ui.label(RichText::new(format!("{:.4} {}", start_price * (p_change / 100.), currency)).heading().color(Color32::RED));
                        } else {
                            ui.label(format!("+{:.2}%", p_change));
                        }
                    });
                    ui.label(RichText::new(format!("Current price: {:.2} {}", latest_price, currency)).strong().heading());
                    
                    ui.horizontal(|ui| {
                        ui.label(metadata.exchange_name.clone());
                        ui.label(metadata.instrument_type.clone());
                    });
                }
                
                self.stock_graph.show(ui);
            });
        });
    }
}
