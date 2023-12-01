#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<ctype.h>

int main()
{
    int i = 0;

    int messageFlag = 1;

    char currentString[5000] = {' '};

    FILE *tuningInput;

    tuningInput = fopen("Day6Input.txt", "r");

    fgets(currentString, 5000, tuningInput);

    while(isalpha(currentString[i]))
    {
        messageFlag = 1;

        printf("%c", currentString[i]);
        
        for(int j = i; j < i + 14; j++)
        {
            for(int k = j; k < i + 13; k++)
            {
                if(currentString[j] == currentString[k+1])
                {
                    messageFlag = 0;
                    break;
                }
            }
            if(messageFlag == 0)
                break;
        }
        
        if(messageFlag == 1)
        {
            printf("\nThe first instance of a  message marker is found after character %d is read", i + 14);
            break;
        }
        i++;
    }

    fclose(tuningInput);

    return 0;
}