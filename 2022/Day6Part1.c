#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<ctype.h>

int main()
{
    int i = 0;

    int characterFlag = 1;

    int markerFlag = 1;

    char currentString[5000] = {' '};

    FILE *tuningInput;

    tuningInput = fopen("Day6Input.txt", "r");

    fgets(currentString, 5000, tuningInput);

    while(isalpha(currentString[i]))
    {
        markerFlag = 1;
        characterFlag = 1;

        printf("%c", currentString[i]);
        
        for(int j = i; j < i + 4; j++)
        {
            for(int k = j; k < i + 3; k++)
            {
                if(currentString[j] == currentString[k+1])
                {
                    markerFlag = 0;
                    break;
                }
            }
            if(markerFlag == 0)
                break;
        }
        
        if(markerFlag == 1)
        {
            printf("\nThe first instance of a marker is found after character %d is read", i + 4);
            break;
        }
        i++;
    }

    fclose(tuningInput);

    return 0;
}