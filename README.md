# Example Rust AWS Lambda

This is an example AWS Lambda written in rust using Apex.

## Deploying

Check you have the apex role and attached policy as named in `project.json`.
This would have been created by `apex init`.
Also update the arn to match your account number and the role name.

Then deploy as usual with `apex deploy`.
This will not print any output on success.
Also it can take a long time if it has to compile a large rust program from scratch
(or one that links to a large library like `rusoto`).

## Apex Runtimes

There are two runtimes: `rust_musl` and `rust_gnu`

These correspond to the two options for `libc`.

See [./functions/hello/function.json](./functions/hello/function.json)

### Musl Libc

For very simple lambdas, you can get good results using `rust_musl`.

This relies on cross-compiling for the target.
At the moment it works on any linux machine or docker container.

It will not have any external dependencies at runtime.
so you can be confident the lambda will work in the target environment.

Some crates don't seem to work such as `error-chain` which uses
`backtrace-sys`. 

Other ffi libraries would bring in a dependency on the system libc
unless you've also cross compiled them to use `musl`. `pkg-config` detects
you're trying to cross compile and balks.

### Gnu Libc

For anything remotely interesting, you should use `rust_gnu`.
You must compile on Amazon Linux to get reliable results.

If you do use Amazon Linux you can link to any library that 
is already installed, such as `libssl`, which means you can use `rust-openssl`.

## Amazon Linux

You'll need the following dependencies:

 * rust toolchain (rustup is probably best)
 * apex (download a release or install golang >=1.7 and go get it)
 * yum packages: glibc-devel gcc gcc-c++
 * optional yum packages: git-core
 * aws credentials or preferrably an ec2 instance role already setup
 
## Copying

Dual licensed `MIT`/`Apache-2`. See license files for details.

