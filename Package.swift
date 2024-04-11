// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let cryptoServiceBinaryTargetName = "CryptoServiceRS"
let binaryTarget: Target
let useLocalFramework = false

if useLocalFramework {
	binaryTarget = .binaryTarget(
		name: cryptoServiceBinaryTargetName,
		// IMPORTANT: Swift packages importing this locally will not be able to
		// import SargonCore unless you specify this as a relative path!
		path: "./target/ios/libcrypto_service-rs.xcframework"
	)
} else {
	let releaseTag = "v0.4.2"
	let releaseChecksum = "05957a40cb544fe8c3de352fd55579582c0a980c912f1b410952164c48a04ad5"
	binaryTarget = .binaryTarget(
		name: cryptoServiceBinaryTargetName,
		url:
			"https://github.com/Vinnstah/crypto-service/releases/download/\(releaseTag)/libcrypto_service-rs.xcframework.zip",
		checksum: releaseChecksum
	)
}

let package = Package(
	name: "CryptoService",
	platforms: [
		.iOS(.v17)
	],
	products: [
		.library(
			name: "CryptoService",
			targets: ["CryptoService"]
		)
	],
	targets: [
		binaryTarget,
		.target(
			name: "CryptoServiceUniFFI",
			dependencies: [.target(name: cryptoServiceBinaryTargetName)],
			path: "apple/Sources/UniFFI"
		),
		.target(
			name: "CryptoService",
			dependencies: ["CryptoServiceUniFFI",],
			path: "apple/Sources/CryptoService"
		),
	]
)