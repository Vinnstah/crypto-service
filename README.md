# Crypto Service
[![codecov](https://codecov.io/gh/Vinnstah/crypto-service/graph/badge.svg?token=YZ6OR1BZXJ)](https://codecov.io/gh/Vinnstah/crypto-service)

A Crypto REST API-backend written in Rust with Axum and uniFFI. 

# Background
This project came to life from wanting to develop a API-backend in Rust and connect it to a iOS-app. 

# How do I use it?
The project can be run locally after adding a `.env` with the API-keys or in the cloud by containerizing it.
There is also the option to bundle the binary directly in your iOS-app by using uniFFI generated bindings.

# Why use uniFFI
The upside of using uniFFI is that all models and subsequently all serialization is done on Rust side. We can keep our frontend free from Models, Methods and Networking which greatly reduces the complexity.

# uniFFI bindings in Swift
 In Rust we define a async trait `with_foreign` meaning that we need to conform to it in Swift in order to inject a Network Antenna from Swift into Rust.

 ```
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
 pub trait NetworkAntenna: Send + Sync {
    async fn make_request(
        &self,
        request: FFINetworkingRequest,
    ) -> Result<FFINetworkingResponse, FFINetworkingError>;

    fn get_api_keys(&self) -> ClientKeys;
}
```

This can be done by extending URLSession and implementing the required methods.

```
extension URLSession: NetworkAntenna {
    public func getApiKeys() -> CryptoServiceUniFFI.ClientKeys {
        URLSession.clientKeys
    }
    
    public func makeRequest(request: CryptoServiceUniFFI.FfiNetworkingRequest) async throws -> CryptoServiceUniFFI.FfiNetworkingResponse {
        guard let url = URL(string: request.url) else {
            return FfiNetworkingResponse.init(statusCode: 500, body: .init())
        }
        
        var urlRequest = URLRequest(url: url)
        
        urlRequest.httpMethod = "POST"
        urlRequest.httpBody = request.body
        urlRequest.allHTTPHeaderFields = request.headers
        
        let (data, response) = try await URLSession.shared.data(for: urlRequest)
        
        guard let httpResponse = response as? HTTPURLResponse,
              (200...299).contains(httpResponse.statusCode) else {
            print(data)
            print(response)
            return FfiNetworkingResponse(statusCode: 400, body: data)
        }
        
        return FfiNetworkingResponse(statusCode: 200, body: data)
    }
}
```

The Network Antenna will be used in Rust to create a `Gateway` which will handle all external API-requests using the injected Network Antenna.

```
#[derive(Object)]
pub struct Gateway {
    pub network_antenna: Arc<dyn NetworkAntenna>,
}
```

We're now able to use our antenna to make network requests in Rust using a externally injected `URLSession`.

```
let response = self
    .network_antenna
    .make_request(request)
    .await?;
```