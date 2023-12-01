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

    char currentString[100] = {' '};

    FILE *stackInput;

    stackInput = fopen("Day5Input.txt", "r");

    //Reads in the original stack state, prints it, and stores it in stackState
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
    printf("\n");
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

        currentStrLen = strlen(currentString);

        //From here to the next comment, it reads in all three integer instructions from the current line and stores them in a 3-long int array.
        for(i = 0; i < currentStrLen-1; i++)
        {
            if(isdigit(currentString[i]))
            {
                currentLineInts[i] = (int)currentString[i] - 48;
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
        //Finished parsing the current instructions.

        //Finds the stack height at each position.
        for(i = 0; i < 9; i++)
        {
            for(j = 0; isalpha(stackState[j][i]); j++)
            {
                stackHeights[i]++;
            }
        }

        //Sets some variables to convenient values for later.
        firstStack = currentInstruct[1] - 1;
        secondStack = currentInstruct[2] - 1;

        iterateUntil = (stackHeights[firstStack]);

        //Does the calculations for where the boxes should go.
        for(i = stackHeights[firstStack] - currentInstruct[0]; i < iterateUntil; i++)
        {
            stackState[stackHeights[secondStack]][secondStack] = stackState[i][firstStack];
            stackHeights[secondStack] = stackHeights[secondStack] + 1;

            stackState[i][firstStack] = ' ';
            stackHeights[firstStack] = stackHeights[firstStack] - 1;
        }
        
        // Sets each stack height to zero before reading them from the next iteration.
        for(i = 0; i < 9; i++)
        {
            stackHeights[i] = 0;
        }

        //Sets the instructions to zero before reading them from the next iteration.
        for(i = 0; i < 3; i++)
        {
            currentInstruct[i] = 0;
        }
    }

    //Prints out the final stack state.
    printf("\n");
    for(int i = 30; i >= 0; i--)
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