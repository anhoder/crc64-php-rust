## crc64 extension for PHP.

Based on [rolling-dual-crc](https://github.com/malaire/rolling-dual-crc)

### Build

> Need to install rust for compilation.

#### Use Cargo

```sh
cargo build --release

# test
cargo build && cargo test
```

`target/release/libcrc64.so`(or `target/release/libcrc64.dylib`) is the compiled extension. 

#### Use Make

```sh
phpize && ./configure && make

# test
make test

make install
```

### Usage

```php
$crc = new Crc\DualCrc();

$crc->update('123456');
printf("0x%x\n", $crc->get64());

$crc->update('7890');
printf("0x%x\n", $crc->get64());
```