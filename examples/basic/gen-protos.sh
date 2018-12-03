#! /bin/bash

protoc -I ../protos \
        -I ../../proto \
        -I /usr/local/include \
        --python_out=src \
        ../protos/*.proto

python -m grpc_tools.protoc \
    -I ../../proto \
    --python_out=src \
    --grpc_python_out=src \
    ../../proto/protodb/*.proto \
    ../../proto/protodb/**/*.proto

generated_paths=( "" "collection/" "database/" "object/" "wasm/" )
for path in "${generated_paths[@]}"
do
    touch src/protodb/${path}__init__.py
done
