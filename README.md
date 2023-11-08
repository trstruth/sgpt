# sgpt

A gpt wrapper which prevents you from having to look up command line syntax.

## Description

`sgpt` is a tiny cli tool which forwards requests to the [chat api](https://platform.openai.com/docs/api-reference/chat/object) and attempts to return valid bash

## Getting Started

1. install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. clone the repo and cd into it: `git clone git@github.com:trstruth/sgpt.git && cd sgpt`
3. [install](https://doc.rust-lang.org/cargo/commands/cargo-install.html) the binary: `cargo install --path .` (ensure the installation root is in your PATH)
4. run sgpt:
    ```
    $ sgpt -- what kubectl command do I run to follow logs from a pod
    kubectl logs -f [pod-name]
    $ sgpt -- post some json to example.com:8080
    curl -X POST -H "Content-Type: application/json" -d '{"key":"value"}' http://example.com:8080
    $ sgpt -- find all files in the current directory which contain the text "sgpt is cool" 
    grep -rl "sgpt is cool" .
    ```