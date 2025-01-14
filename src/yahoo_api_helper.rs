use std::thread::{JoinHandle, self};

use yahoo::{YahooError, YResponse, YahooConnector, YMetaData, YSearchResult};
use yahoo_finance_api as yahoo;

pub type YahooFetchHandle = Option<JoinHandle<Result<YResponse, YahooError>>>;
pub type YahooFetchSearchHandle = Option<JoinHandle<Result<YSearchResult, YahooError>>>;

pub fn fetch_recent_interval(fetch_handle: &mut YahooFetchHandle, ticker: &String, stock_data: &mut Vec<[f64; 2]>, volume_data: &mut Vec<[f64; 2]>, metadata: &mut Option<YMetaData>) {
    if fetch_handle.is_none() {
        let tick = ticker.clone();
        let handle = thread::spawn(move || {
            let provider = YahooConnector::new().unwrap();
            let response = provider.get_quote_period_interval(&tick, "max", "1m", false);

            response
        });

        *fetch_handle = Some(handle);
    }

    let fh_temp = fetch_handle.take();
    if let Some(handle) = fh_temp {
        if handle.is_finished() {
            let response = handle.join().unwrap();

			stock_data.clear();
            volume_data.clear();

            match response {
                Ok(resp) => {
                    for quote in resp.quotes().unwrap() {
                        stock_data.push([quote.timestamp as f64, quote.close]);
                        volume_data.push([quote.timestamp as f64, quote.volume as f64]);
                    }

                    *metadata = Some(resp.metadata().unwrap());
                },
                Err(_) => {
					eprintln!("Error stock '{ticker}' not found");
                    *metadata = None;
                },
            }
        } else {
            *fetch_handle = Some(handle);
        }
    }
}

pub fn fetch_history(fetch_handle: &mut YahooFetchHandle, ticker: &String, range: &String, stock_data: &mut Vec<[f64; 2]>, volume_data: &mut Vec<[f64; 2]>, metadata: &mut Option<YMetaData>) {
    if fetch_handle.is_none() {
        let tick = ticker.clone();
        let range = range.clone();
        let handle = thread::spawn(move || {
            let provider = YahooConnector::new().unwrap();
            let response = provider.get_quote_range(&tick, "1d", &range);

            response
        });

        *fetch_handle = Some(handle);
    }

    let fh_temp = fetch_handle.take();
    if let Some(handle) = fh_temp {
        if handle.is_finished() {
            let response = handle.join().unwrap();

			stock_data.clear();
            volume_data.clear();

            match response {
                Ok(resp) => {
                    for quote in resp.quotes().unwrap() {
                        stock_data.push([quote.timestamp as f64, quote.close]);
                        volume_data.push([quote.timestamp as f64, quote.volume as f64]);
                    }

                    *metadata = Some(resp.metadata().unwrap());
                },
                Err(_) => {
					eprintln!("Error stock '{ticker}' not found");
                    *metadata = None;
                },
            }
        } else {
            *fetch_handle = Some(handle);
        }
    }
}


pub fn fetch_now_data(fetch_handle: &mut YahooFetchHandle, ticker: &String, latest_price: &mut f64, metadata: &mut Option<YMetaData>) {
    if fetch_handle.is_none() {
        let tick = ticker.clone();
        let handle = thread::spawn(move || {
            let provider = YahooConnector::new().unwrap();
            let response = provider.get_quote_period_interval(&tick, "max", "1m", false);

            response
        });

        *fetch_handle = Some(handle);
    }

    let fh_temp = fetch_handle.take();
    if let Some(handle) = fh_temp {
        if handle.is_finished() {
            let response = handle.join().unwrap();

            match response {
                Ok(resp) => {
                    *latest_price = resp.last_quote().unwrap().close;
                    *metadata = Some(resp.metadata().unwrap());
                },
                Err(_) => {
					eprintln!("Error stock '{ticker}' not found");
                    *latest_price = 0.;
                    *metadata = None;
                },
            }
        } else {
            *fetch_handle = Some(handle);
        }
    }
}

pub fn fetch_search_ticker(fetch_handle: &mut YahooFetchSearchHandle, search: &String, search_result: &mut Option<YSearchResult>, found_result: &mut bool) {
    if fetch_handle.is_none() {
        let search = search.clone();
        let handle = thread::spawn(move || {
            let provider = YahooConnector::new().unwrap();
            let response = provider.search_ticker(&search);

            response
        });

        *fetch_handle = Some(handle);
    }

    let fh_temp = fetch_handle.take();
    if let Some(handle) = fh_temp {
        if handle.is_finished() {
            let response = handle.join().unwrap();

            *found_result = true;

            *search_result = match response {
                Ok(r) => Some(r),
                Err(_) => None,
            }
        } else {
            *fetch_handle = Some(handle);
        }
    }
}
