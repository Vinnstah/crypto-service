// public struct Client {
//   let binanceKey: String
//   let coinApiKey: String

//   public init(binanceKey: String, coinApiKey: String) {
//     self.binanceKey = binanceKey
//     self.coinApiKey = coinApiKey
//   }
// }

// extension Client {
//     public func getSymbols(params: SymbolsParams) async -> [SymbolsResponse]  {
//       await getSymbolsBinding(params: params) 
//     }

//     public func getOrderbook(params: Params) async -> OrderBook {
//       await getOrderbookBinding(params: params, binanceKey: self.binanceKey, coinKey: self.coinApiKey) 
//     }
// }

public extension AggregatedCoinInformation: Codable {}
public extension ListOfCoinsRequest: Codable {}
public extension CoinHistoryRequest: Codable {}
public extension CoinMeta: Codable {}
public extension Coin: Codable {}