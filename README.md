# grarr

[grarr][] is a git repository and reviews renderer. It can be run on a personal
server to provide a nice web interface to view your self-hosted git repositories
without the overhead of a full on git repository management tool like
[GitLab][].

[grarr]: https://git.nemo157.com/grarr
[GitLab]: https://gitlab.com

## TODO

 * Implement [smart http protocol][] so things like `cargo install --git
   https://git.nemo157.com/grarr` will work. (Cargo uses libgit2 which doesn't
   support the dumb http protocol. May as well support the smart http protocol
   (assuming it's not too difficult) instead of requiring a separate http server
   for it.)

[smart http protocol]: https://github.com/git/git/blob/master/Documentation/technical/http-protocol.txt

## Developing

If building on OS X with a `homebrew` installed copy of OpenSSL you'll need to
specify where this is to enable building `libssh2-sys` and `openssl-sys-extras`.
Use something like:

```sh
OPENSSL_ROOT_DIR=`brew --prefix openssl` \
OPENSSL_LIB_DIR=`brew --prefix openssl`/lib \
OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include \
cargo build
```
