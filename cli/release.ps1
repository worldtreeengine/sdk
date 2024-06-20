cargo build -r;
cargo about generate -o target/release/THIRD-PARTY about.hbs
$metadata = cargo metadata --no-deps --format-version=1 | ConvertFrom-Json;
$version = $metadata.packages[0].version;
iscc installers/windows/worlds.iss /DApplicationVersion=$version;
