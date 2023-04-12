
#include <stdio.h>
#include <stdint.h>
#include "log_txt_c.h"

int main() {
    int32_t a = 3;
    int32_t b = 4;
    int32_t result = add(a, b);
    printf("add(%d, %d) = %d\n", a, b, result);

    const char *file_name = "test.txt";
    const char *content = "Hello, world!";
    int32_t success = append_to_file(file_name, content);
    if (success == 0) {
        printf("Successfully appended '%s' to file '%s'\n", content, file_name);
    } else {
        printf("Failed to append '%s' to file '%s'\n", content, file_name);
    }

    char *file_contents = read_from_file(file_name);
    if (file_contents != NULL) {
        printf("Contents of file '%s':\n%s\n", file_name, file_contents);
    } else {
        printf("Failed to read from file '%s'\n", file_name);
    }

    return 0;
}