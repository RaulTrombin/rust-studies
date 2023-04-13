cbindgen --config cbindgen.toml --crate log_txt_c --output my_header.h

How to test using C

gcc -o test_log_txt_c test_log_txt_c.c -L./target/debug -llog_txt_c
export LD_LIBRARY_PATH=target/debug:$LD_LIBRARY_PATH
./test_log_txt_c

How to test using python ctypes

python
