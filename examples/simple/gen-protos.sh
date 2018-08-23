#! /bin/bash

mkdir -p src

python -m grpc_tools.protoc \
    -I ../../proto \
    --python_out=src \
    --grpc_python_out=src \
    ../../proto/protodb/*.proto

touch src/protodb/__init__.py

