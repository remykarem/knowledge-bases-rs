# KB

1. Download Protobuf.

    ```sh
    brew install protobuf
    ```

    Protobuf is needed for gRPC with the vector database.

2. Download libtorch v1.31.1 for macOS CPU from [here](https://download.pytorch.org/libtorch/cpu/libtorch-macos-1.31.1.zip). Then unzip it.

3. Set the following env variables:
  
    ```sh
    export LIBTORCH=<path-to-unzipped-folder>
    export DYLD_FALLBACK_LIBRARY_PATH=${LIBTORCH}/lib  # for macOS
    ```

    For other platforms, see [here](https://doc.rust-lang.org/cargo/reference/environment-variables.html#dynamic-library-paths) to specify dynamic library path.

3. Run the Qdrant container.

    ```sh
    docker run -d -p 6333:6333 -p 6334:6334 \
        -v $(pwd)/qdrant_storage:/qdrant/storage \
        qdrant/qdrant
    ```

4. Run

    ```sh
    cargo run
    ```
