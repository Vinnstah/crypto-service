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
	let releaseTag = "v0.0.6"
	let releaseChecksum = "ef568bfebe64fb75544a45ef5fb8cb214822588a9b79170834507831de92a7b9"
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