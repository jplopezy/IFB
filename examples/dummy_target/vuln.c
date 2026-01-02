#include <stddef.h>
#include <string.h>

int process_data(const char *data, size_t len) {
    char buffer[8];

    if (len >= 4 && memcmp(data, "BOOM", 4) == 0) {
        // Intentional overflow when the trigger word is present.
        memcpy(buffer, data, len);
        return buffer[0];
    }

    return 0;
}
