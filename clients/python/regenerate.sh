#! /bin/bash

set -e

python -m grpc_tools.protoc \
    -I ../../proto \
    --python_out=. \
    --grpc_python_out=. \
    ../../proto/protodb/*.proto \
    ../../proto/protodb/**/*.proto

generated_paths=( "" "collection/" "database/" "index/" "object/" "wasm/" )
for path in "${generated_paths[@]}"
do
    touch protodb/${path}__init__.py
done
