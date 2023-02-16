Run rust as part of the meson build system
This repo is an example of two way referencing.
It references `csp` using bindgen and is referenced by `csh` using static linking

# Compilation
Rust is compiled using a CustomTarget. Meson does provide their own way of doing it. However that circumvents Cargo.toml and requires for all source files to be added manually.

The CustomTarget uses cargo and therefore any argument that is normally passed to cargo can be setup to be accepted by meson and then passed to cargo if needed.

# Referencing csp
all the csp functions are available in the csp module found in the root.
```rust
pub use crate::csp::*;
```
All the methods are raw ffi bindings so it is a good idea to create safe rust wrappers around them before use.
## How it works
Getting csp into rust is a two step process.

Firstly rust header are generated using bindgen. I am using the building bindgen for meson but there is nothing stopping us from using a build script instead.

The main benefit of using meson is that we do not have to worry about locating csp as it takes care of it for us. Unfortunalty it also runs the generation every build which might be slow (I guess it is relative).
> running bindgen requires for clang to be installer. Install using `sudo apt install clang`

The second step is to link with csp. This is handled by meson when csp is specified as a dependency
```
csp_dep = dependency('csp', fallback : ['csp', 'csp_dep'])
```

# Calling rust from csh
To add this example to csh just add this repo to the lib folder and as a dependency to csh in the meson.build found in the root of csh.
```
csp_to_csh_example_dep = dependency('csp_to_csh_example', fallback: ['csp_to_csh_example', 'csp_to_csh_example_dep'], required: true)

...

csh = executable(
    'csh',
    ..
	dependencies : [.. csp_to_csh_example_dep],
	..
)
```
> One sidenote csh has `.as_link_whole()` at the end of all its dependencies. However doing this on a CustomTarget results in an error. So it is left to avoid trouble.

The extern functions in rust can then be called by importing the headers
```c
#include <csp_to_csh_example/csp_to_csh_example.h>
```

> NOTICE: When making changes to the rust code, the headers have to be updated manually. (cbindgen could automate this potentially)

## How it works
Rust is compiled as a static library
> NOTICE: We can also compile it as a dynamic library, but then we have to make sure that the library is there at runtime.
```toml
[lib]
name = "csp_to_csh_example"
crate-type = ["staticlib"]
```
The compilation is done using a CustomTarget. 

The CustomTarget allows us to run `cargo build` just like it is done in the commandline. Meson does come with their own wrapper around cargo. However does not use `Cargo.toml` and requires manual tracking of files. Which seem highly uncessecary.

> We can pass arguments to `cargo build` by setting up option in meson for inspiration on how this is done look at profile in `meson_options.txt`

Once it compilation is done, the library file is copied into mesons build directory.

Then to tell meson that the library exist we create the dependency declaration. The way this works is a bit of a hack since it seems like no one is really sure how it is suppose to work. The solution used is based on [this comment](https://github.com/mesonbuild/meson/issues/3613#issuecomment-408276296) in the issue tracking the problem.