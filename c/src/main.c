#include <stdio.h>

int main(void) {
   
    char file_path[] = "../files/day1.txt";
    FILE *fp = fopen(file_path, "r");
    char buff[500];

    if (fp == NULL) {
        printf("File %s not found", file_path);
        return 1;
    }

    fscanf(fp, "%s", buff);
    fgets(buff, 100, (FILE*)fp);
    
    char c;
    int n_lines = 1;
    while ( fscanf(fp, "%c", &c) == 1 ) {
        printf("%c", c);
        if (c == '\n') {
            n_lines += 1;
        }
    }

    printf("\n%d lines",n_lines);

    return 0;
}
