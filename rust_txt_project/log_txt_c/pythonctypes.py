import sys
import ctypes
from pprint import pprint

lib=ctypes.CDLL("target/debug/liblog_txt_c.so")

# lib.append_to_file("crates.txt","text")
# print(lib.read_from_file("crates.txt"))

# Set the argument and return types for append_to_file
lib.append_to_file.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
lib.append_to_file.restype = ctypes.c_int

# Call append_to_file
lib.append_to_file(b"test.txt", b"text")

# Set the argument and return types for read_from_file
lib.read_from_file.argtypes = [ctypes.c_char_p]
lib.read_from_file.restype = ctypes.c_char_p

# Call read_from_file and print the result
result = lib.read_from_file(b"test.txt")
print(result.decode())

