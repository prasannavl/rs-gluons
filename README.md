# rust-play

Rust Playground

- **hello-rs-nostd**  
  - Smallest possible statically linked hello world
  - No std-lib
  - Console subsystem 
  - Size: 10k

- **hello-rs-nostd-win**  
  - Smallest possible statically linked hello world
  - No std-lib
  - Windows subsystem
  - Size: 10k
  - Handles windows manifest embed via build script (There's no coherent cargo idiomatic build process to orchestrate post-build, or win32 manifest embedding at the moment of writing this - Hopefully that should change soon)

