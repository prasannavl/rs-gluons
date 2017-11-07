# rs-gluons

A grab bag of personal rust modules, and templates to hold things together

### Modules
- **sort** - Efficient sorting algorithms (WIP)
- **search** - Search algorithms (WIP)
- [TODO] **collections**

### Templates

- **hello-win-nostd**  
  - Smallest possible statically linked hello world
  - No std-lib
  - Console subsystem 
  - Size: 10k

- **hello-win-nostd-gui**  
  - Smallest possible statically linked hello world
  - No std-lib
  - Windows (GUI) subsystem
  - Size: 10k
  - Handles windows manifest embed via build script (There's no coherent cargo idiomatic build process to orchestrate post-build, or win32 manifest embedding at the moment of writing this - Hopefully that should change soon)
