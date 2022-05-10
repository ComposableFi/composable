
# Download a composable Relay Chain
# Runs picasso or composable relay chains
mkdir -p ../../../composable/target/release
curl --location https://storage.googleapis.com/composable-binaries/community-releases/picasso/composable-picasso-599ddfcb20ed6efb82d826c5367b572ddb338878.tar.gz --output ../../../composable/target/release/composable.tar.gz
tar -xf  ../../../composable/target/release/composable.tar.gz -C ../../../composable/target/release/
sudo chmod +x ../../../composable/target/release/composable
../../../composable/target/release/composable --version
