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

    int iterateUntil = 0;

    int firstStack = 0;
    int secondStack = 0;

    int currentLineInts[20] = {0};

    int currentInstruct[3] = {0};

    int stackHeights[9] = {0};

    char stackState[100][100] = {' '};

    char currentString[100];

    FILE *stackInput;

    stackInput = fopen("Day5Input.txt", "r");

    //Reads in the original stack state and stores it in stackState
    for(int i = 7; i >= 0; i--)
    {
        fgets(currentString, 100, stackInput);
        printf("%s", currentString);

        for(int j = 2; j < 35; j = j + 4)
        {
            stackState[i][(j-2)/4] = currentString[j-1];
        }
    }
    
    //Prints out the original stackState
    for(int i = 7; i >= 0; i--)
    {
        for(int j = 0; j < 9; j++)
        {
            printf("%c ",stackState[i][j]);
        }
        printf("\n");
    }


    //Takes in the rest of the file and parses it.
    while(fgets(currentString, 100, stackInput) != NULL)
    {
        if(currentString[0] != 'm')
            continue;
        // fgets(currentString, 100, stackInput);

        // printf("%s", currentString);

        currentStrLen = strlen(currentString);

        for(i = 0; i < currentStrLen-1; i++)
        {
            if(isdigit(currentString[i]))
            {
                currentLineInts[i] = (int)currentString[i] - 48;
                // printf("%d", currentLineInts[i]);
            }
            else currentLineInts[i] = 0;
        }

        k = 0;
        j = currentStrLen-1;
        for(i = 3; i >=0; i--)
        {
            while(isdigit(currentString[j]))
            {
                currentInstruct[i] = currentInstruct[i] + currentLineInts[j]*pow((int)10, k);
                k++;
                j--;
            }
            while(!isdigit(currentString[j]))
            {
                j--;
            }
            k = 0;
        }

        //Finds the stack height at each position.
        for(i = 0; i < 9; i++)
        {
            for(j = 0; isalpha(stackState[j][i]); j++)
            {
                stackHeights[i]++;
            }
        }

        firstStack = currentInstruct[1] - 1;
        secondStack = currentInstruct[2] - 1;

        iterateUntil = (stackHeights[firstStack]-1)-currentInstruct[0];
        
        for(i = stackHeights[firstStack] - 1; i > iterateUntil; i--)
        {
            stackState[stackHeights[secondStack]][secondStack] = stackState[i][firstStack];
            stackHeights[secondStack] = stackHeights[secondStack] + 1;

            stackState[i][firstStack] = ' ';
            stackHeights[firstStack] = stackHeights[firstStack] - 1;
        }
        
        // Prints the stack height at each position
        // printf("\n");
        for(i = 0; i < 9; i++)
        {
            stackHeights[i] = 0;
        }

        for(i = 0; i < 3; i++)
        {
            // printf("%d ", currentInstruct[i]);

            currentInstruct[i] = 0;
        }
        // printf("\n");
    }

    printf("\n");
    for(int i = 20; i >= 0; i--)
    {
        for(int j = 0; j < 9; j++)
        {
            printf("%c ",stackState[i][j]);
        }
        printf("\n");
    }

    fclose(stackInput);

    return 0;
}