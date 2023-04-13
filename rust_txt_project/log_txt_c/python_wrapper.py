import ctypes

# Load the shared library
lib = ctypes.CDLL("target/debug/liblog_txt_c.so")

# Define the argument and return types for append_to_file
lib.append_to_file.argtypes = (ctypes.c_char_p, ctypes.c_char_p)
lib.append_to_file.restype = ctypes.c_int

# Define the argument and return types for read_from_file
lib.read_from_file.argtypes = (ctypes.c_char_p,)
lib.read_from_file.restype = ctypes.c_char_p

# Define a function to append text to a file
def append_to_file(filename, text):
    lib.append_to_file(filename.encode(), text.encode())

# Define a function to read text from a file
def read_from_file(filename):
    result = lib.read_from_file(filename.encode())
    return result.decode()