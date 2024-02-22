public struct Client {
  public init() {}
}

extension Client {
    public func getSymbols(params: SymbolsParams) async -> [SymbolsResponse]  {
      await getSymbolsBinding(params: params) 
    }

    public func getOrderbook(params: Params, key: String) async -> OrderBook {
      await getOrderbookBinding(params: params, key: key) 
    }
}
