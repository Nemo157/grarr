# grarr

[`grarr`][] is a git repository and reviews renderer. It can be run on a personal
server to provide a nice web interface to view your self-hosted git repositories
without the overhead of a full on git repository management tool like [GitLab][].

## Developing

If building on OS X with a `homebrew` installed copy of OpenSSL you'll need to
specify where this is to enable building `libssh2-sys` and `openssl-sys-extras`.
Use something like:

```sh
OPENSSL_ROOT_DIR=/usr/local/opt/openssl \
DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include \
cargo build
```

[`grarr`]: https://grarr.nemo157.com/grarr
[GitLab]: https://gitlab.com
