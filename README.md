# rustllm_api
Using nvidia nim free api credits to create a rust program that is able to parse the input and generate answers

# Rust Command Line API Client for NVIDIA NIMS

This Rust application, `rustllm_api`, is a command line tool that sends data via API to NVIDIA's Metropolis Inference Service (NIMS) and generates an answer. It takes arguments from the command line to configure the API request.

https://www.nvidia.com/en-us/ai/
 Create a free account and get free api credits. The get the API key from the link on your page and enter it in main.rs, line 40.


## Building

To build the application, you'll need to have Rust and Cargo installed. Clone this repository and run:


    git clone https://github.com/pnmeka/rustllm_api

    cd rustllm_api

## COnfiguration
Get an API from Nvidia Nims and enter in line 40 in src/main.rs

    .bearer_auth("$ENTER THE API HERE FROM NVIDIA NIM")

## Usage
  

    cargo run Hello LLM

Once you have the binary ready:

    ./rustllm_api $write a message

    ./rustllm_api Write a python code to generate a qr code from hello world

