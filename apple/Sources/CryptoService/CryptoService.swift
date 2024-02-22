public struct Client {
  let binanceKey: String
  let coinApiKey: String

  public init(binanceKey: String, coinApiKey: String) {
    self.binanceKey = binanceKey
    self.coinApiKey = coinApiKey
  }
}

extension Client {
    public func getSymbols(params: SymbolsParams) async -> [SymbolsResponse]  {
      await getSymbolsBinding(params: params) 
    }

    public func getOrderbook(params: Params) async -> OrderBook {
      await getOrderbookBinding(params: params, binance_key: self.binanceKey, coin_key: self.coinApiKey) 
    }
}

// Fix API-KEY for CoinApi since we instantiate both when creating the CLient for State. 