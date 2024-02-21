import UniFFi

public struct Client {}

extension Client {
    public func getSymbols(params: UniFFi.SymbolsParams) async -> [UniFFI.SymbolsResponse]  {
      UniFFi.getSymbolsBinding(params) 
    }
}