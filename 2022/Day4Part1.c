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

    int totalContainment = 0;

    int currentLineInts[20] = {0};

    int allTasks[4] = {0};

    FILE *cleanupInput;

    cleanupInput = fopen("Day4Input.txt", "r");

    while(fgets(currentString, 20, cleanupInput) != NULL)
    {

    
        // fgets(currentString, 20, cleanupInput);

        printf("\n%s", currentString);

        currentStrLen = strlen(currentString);

        for(i = 0; i < currentStrLen-1; i++)
        {
            if(isdigit(currentString[i]))
            {
                currentLineInts[i] = (int)currentString[i] - 48;
            }
            // printf("%d", currentLineInts[i]);
        }

        k = 0;
        j = currentStrLen-1;
        for(i = 4; i >=0; i--)
        {
            while(isdigit(currentString[j]))
            {
                // printf("\nInner Loop #%d", j);
                allTasks[i] = allTasks[i] + currentLineInts[j]*pow((int)10, k);
                k++;
                j--;
                // printf("\n%d",j);
            }
            // printf("\nOuter Loop #%d", i);
            j--;
            // printf("\n%d",j);
            // printf("\nSet k to zero");
            k = 0;
        }

        if(allTasks[0] <= allTasks[2] && allTasks[1] >= allTasks[3])
        {
            printf("\nContained.");
            totalContainment++;
        }
                
    
        else if(allTasks[2] <= allTasks[0] && allTasks[3] >= allTasks[1])
        {
            printf("\nContained.");
            totalContainment++;
        }
            

        printf("\n");
        for(i = 0; i < 4; i++)
        {
            printf("%d ", allTasks[i]);

            allTasks[i] = 0;
        }

    }

    fclose(cleanupInput);

    printf("\nThe total amount of pairs with a set of tasks fully contained by the other is %d", totalContainment);

    return 0;
}