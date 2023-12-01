#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<ctype.h>
#include<math.h>

int main()
{
    int i = 0;
    int j = 0;
    int k = 0;
    
    int currentStrLen = 0;

    char currentString[20];

    int totalOverlap = 0;

    int currentLineInts[20] = {0};

    int allTasks[4] = {0};

    FILE *cleanupInput;

    cleanupInput = fopen("Day4Input.txt", "r");

    while(fgets(currentString, 20, cleanupInput) != NULL)
    {
        printf("\n%s", currentString);

        currentStrLen = strlen(currentString);

        for(i = 0; i < currentStrLen-1; i++)
        {
            if(isdigit(currentString[i]))
            {
                currentLineInts[i] = (int)currentString[i] - 48;
            }
        }

        k = 0;
        j = currentStrLen-1;
        for(i = 4; i >=0; i--)
        {
            while(isdigit(currentString[j]))
            {
                allTasks[i] = allTasks[i] + currentLineInts[j]*pow((int)10, k);
                k++;
                j--;
            }
           
            j--;
            k = 0;
        }

        //Detects overlap the slow way because I can't be bothered to think of a fast algorithm.
        for(i = allTasks[0]; i <= allTasks[1]; i++)
        {
            for(j = allTasks[2]; j <= allTasks[3]; j++)
            {
                if(i == j)
                {
                    totalOverlap++;
                    i = 10000;
                    j = 10000;
                }
            }
        }
        for(i = 0; i < 4; i++)
        {
            printf("%d ", allTasks[i]);

            allTasks[i] = 0;
        }

    }

    fclose(cleanupInput);

    printf("\nThe total amount of pairs with overlap is %d", totalOverlap);

    return 0;
}