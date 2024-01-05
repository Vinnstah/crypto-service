use super::orderbook_handler::OrderBookRequest;

pub trait QueryItem {
    fn get_symbol(&self) -> String;
    fn get_limit(&self) -> Option<u16>;
}

impl QueryItem for OrderBookRequest {
    fn get_symbol(&self) -> String {
        self.symbol.clone()
    }

    fn get_limit(&self) -> Option<u16> {
        if let Some(limit) = self.limit {
            return Some(limit);
        }
        None
    }
}
