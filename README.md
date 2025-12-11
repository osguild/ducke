# Install Process

1. `git clone https://github.com/osguild/ducke`

2. `cd ducke`

3. For debug (or to build locally): `cargo build` 
   For release: `cargo build --release`


4. To run debug : `./target/debug/ducke` 
   To run release : `./target/release/ducke` 

5. cd `/target/release`
   pwd

6. To add the binary to your path (macos) : `echo 'export PATH="/Users/ian/repos/ducke/target/release:$PATH"' >> ~/.zshrc`

7. TODO: Add GHA to publish the binary to latest git release tag.
