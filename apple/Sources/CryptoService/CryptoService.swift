import CryptoServiceUniFFI

public struct Client {}

extension Client {
    public func getSymbols(params: SymbolsParams) async -> [SymbolsResponse]  {
      getSymbolsBinding(params: params) 
    }

    public func getOrderbook(params: Params) -> OrderBookResponse {
      getOrderbookBinding(params: params) 
    }
}