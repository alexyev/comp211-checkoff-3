# comp211-checkoff-3
Public repository containing my COMP211 Checkoff 3 requirements.

# To Run The Code in this Repo
### Rust
To run the Rust code in the repo, you must first download XCode CLI and Rust. The below instructions are for Mac. If you're running on Linux or Windows, please follow the appopriate instructions on the [Rust Website](rustup.rs).

Install Xcode CLI with this command: 
`xcode-select --install`

Install Rust: 
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Close and reopen the terminal once these two commands have run. Finally, compile with:
`rustc --edition 2024 example.rs -o example-rust`

Run the executable with: 
`./example-rust`

### C 
To run the C code in this repo, please make sure you've downloaded XCode CLI tools as outlined above. Again, these instructions are for Mac. If you're running Linux or Windows, please follow the appropriate instructions outlined in this guide by [Visual Studio Code](https://code.visualstudio.com/docs/languages/cpp).

Make sure that Clang is installed: 
`clang --version`

Compile using Clang: 
`clang -Wall -Wextra example.c -o example-c`

Run the executable with: 
`./example-c`
