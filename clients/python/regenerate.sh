#! /bin/bash

set -e

python -m grpc_tools.protoc \
    -I ../../protos \
    --python_out=. \
    --grpc_python_out=. \
    ../../protos/protodb/*.proto \
    ../../protos/protodb/**/*.proto

generated_paths=( "" "collection/" "database/" "index/" "object/" "wasm/" )
for path in "${generated_paths[@]}"
do
    touch protodb/${path}__init__.py
done
