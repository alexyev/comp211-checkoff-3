#include <stdio.h>   
#include <stdlib.h>  
#include <string.h> 

typedef struct {
    char *name;      
} Buffer;

Buffer buffer_new(const char *text) {
    Buffer buf;
    buf.name = malloc(strlen(text) + 1);   
    if (buf.name == NULL) {
        exit(1);                           
    }
    strcpy(buf.name, text);
    return buf;
}

void buffer_drop(Buffer *self) {
    printf("Releasing %s\n", self->name);
    free(self->name);
    self->name = NULL;
}

int main(void) {
    Buffer a = buffer_new("a");
    printf("Created %s\n", a.name);

    {
        Buffer b = buffer_new("b");
        printf("Created %s\n", b.name);
        buffer_drop(&b);          
    }

    printf("End of main\n");
    buffer_drop(&a);           
    return 0;
}