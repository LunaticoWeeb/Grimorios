#include <stdio.h>
#include <stdlib.h>
#include "Util.h"

void boolean_print(boolean bool)
{
    if(bool == TRUE)
    {
        printf("TRUE\n");
    }
    else if(bool == FALSE)
    {
        printf("FALSE\n");
    }
    else
    {
        printf("ERRO\n");
    }
}

char *readLine() {
    char *string = NULL;
    char currentInput;
    int index = 0;
    do {
        currentInput = (char)getchar();
        string = (char *) realloc(string, sizeof(char) * (index + 1));
        string[index] = currentInput;
        index++;
        if(currentInput == '\r')
        {
            currentInput = (char)getchar();
        }
    } while((currentInput != '\n') && (currentInput != EOF));
    string[index - 1] = '\0';
    return string;
}