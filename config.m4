PHP_ARG_ENABLE([php_add],
  [whether to enable php_add support],
  [AS_HELP_STRING([--enable-php_add],
    [Enable php_add support])],
  [no])

dnl If not enable, `cargo build` run with argument `--release`.
PHP_ARG_ENABLE([cargo_debug], [whether to enable cargo debug mode],
[  --enable-cargo-debug           Enable cargo debug], no, no)

if test "$PHP_php_add" != "no"; then
  dnl Check cargo command exists or not.
  AC_PATH_PROG(CARGO, cargo, no)
  if ! test -x "$CARGO"; then
    AC_MSG_ERROR([cargo command missing, please reinstall the cargo distribution])
  fi

  AC_DEFINE(HAVE_php_add, 1, [ Have php_add support ])

  PHP_NEW_EXTENSION(php_add, [ ], $ext_shared)

  CARGO_MODE_FLAGS="--release"
  CARGO_MODE_DIR="release"

  if test "$PHP_CARGO_DEBUG" != "no"; then
    CARGO_MODE_FLAGS=""
    CARGO_MODE_DIR="debug"
  fi

  cat >>Makefile.objects<< EOF
all: cargo_build

clean: cargo_clean

cargo_build:
    # Build the extension file
	PHP_CONFIG=$PHP_PHP_CONFIG cargo build $CARGO_MODE_FLAGS

    # Copy the extension file from target dir to modules
	if [[ -f ./target/$CARGO_MODE_DIR/libphp_add.dylib ]] ; then \\
		cp ./target/$CARGO_MODE_DIR/libphp_add.dylib ./modules/php_add.so ; fi
	if [[ -f ./target/$CARGO_MODE_DIR/libphp_add.so ]] ; then \\
		cp ./target/$CARGO_MODE_DIR/libphp_add.so ./modules/php_add.so ; fi

cargo_clean:
	cargo clean

.PHONY: cargo_build cargo_clean
EOF

  dnl Symbolic link the files for `cargo build`
  AC_CONFIG_LINKS([ \
    Cargo.lock:Cargo.lock \
    Cargo.toml:Cargo.toml \
    build.rs:build.rs \
    src:src \
    ])
fi
