#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

int32_t append_to_file(const char *file_name, const char *content);

char *read_from_file(const char *file_name);

} // extern "C"
