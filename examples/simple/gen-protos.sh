#! /bin/bash

mkdir -p src

python -m grpc_tools.protoc \
    -I ../../proto \
    --python_out=src \
    --grpc_python_out=src \
    ../../proto/protod/*.proto

touch src/protod/__init__.py

