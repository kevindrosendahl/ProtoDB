#! /bin/bash

protoc -I src/protos \
        -I ../../proto \
        --python_out=src \
        src/protos/*.proto

python -m grpc_tools.protoc \
    -I ../../proto \
    --python_out=src \
    --grpc_python_out=src \
    ../../proto/protodb/*.proto

touch src/protodb/__init__.py

