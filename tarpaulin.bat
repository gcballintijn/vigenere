docker run --security-opt seccomp=unconfined -v "C:\users\gerco\workspaces\rust\vigenere:/volume" xd009642/tarpaulin:0.20.0-slim cargo tarpaulin -o html