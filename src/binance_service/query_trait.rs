use super::orderbook_handler::OrderBookRequest;

pub trait QueryItem {
    fn get_symbol(&self) -> String;
    fn get_limit(&self) -> Option<u16>;
}

// impl QueryItem for OrderBookRequest {
//     fn get_symbol(&self) -> String {
//         self.symbol.clone()
//     }

//     fn get_limit(&self) -> Option<u16> {
//         match self.limit {
//             Some(limit) => Some(limit),
//             None => None,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn get_symbol() {
//         let query_request = OrderBookRequest {
//             symbol: "ETHUSDT".to_string(),
//             limit: None,
//         };
//         assert_eq!(query_request.get_symbol(), "ETHUSDT".to_string())
//     }

//     #[test]
//     fn get_limit() {
//         let query_request = OrderBookRequest {
//             symbol: "ETHUSDT".to_string(),
//             limit: Some(11),
//         };
//         assert_eq!(query_request.get_limit(), Some(11))
//     }

//     #[test]
//     fn get_limit_return_none() {
//         let query_request = OrderBookRequest {
//             symbol: "ETHUSDT".to_string(),
//             limit: None,
//         };
//         assert_eq!(query_request.get_limit(), None)
//     }
// }
