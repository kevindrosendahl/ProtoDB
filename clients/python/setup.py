import setuptools

setuptools.setup(
    name="protodb",
    version="0.0.1",
    author="Kevin Rosendahl",
    author_email="kevindrosendahl@gmail.com",
    description="Python client for ProtoDB",
    url="https://github.com/kevindrosendahl/ProtoDB",
    install_requires=[
        "googleapis-common-protos",
        "grpcio",
        "grpcio-tools",
        "protobuf",
        "six",
    ],
    packages=setuptools.find_packages(),
)