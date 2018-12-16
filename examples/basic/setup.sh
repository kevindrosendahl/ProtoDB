#! /bin/bash

set -e

pushd ../../clients/python
python setup.py sdist
popd

mkdir -p user

protoc -I ../protos \
        -I /usr/local/include \
        --python_out=user \
        ../protos/*.proto

pip install -r requirements.txt

touch user/__init__.py
