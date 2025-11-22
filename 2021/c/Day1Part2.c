#include<stdio.h>
#include<stdlib.h>

int main()
{
    char currentLine[20];

    int window[4];

    int previousBlock = 0;

    int currentBlock = 0;

    int totalIncrease = 0;

    FILE *sonarInput;

    sonarInput = fopen("Day1Input.txt", "r");

    //Increments fgets() once to skip the first line, since the first line
    fgets(currentLine, 20, sonarInput);

    for(int i = 0; i < 3; i++)
    {
        window[i] = atoi(fgets(currentLine, 20, sonarInput));
    }

    while(fgets(currentLine, 20, sonarInput) != NULL)
    {
        window[3] = atoi(currentLine);

        for(int i = 0; i <3; i++)
        {
            previousBlock = previousBlock + window[i];
            currentBlock = currentBlock + window[i + 1];
        }

        if(currentBlock > previousBlock)
        {
            totalIncrease++;
        }
        currentBlock = 0;
        previousBlock = 0;

        for(int i = 0; i < 3; i++)
        {
            window[i] = window[i+1];
        }
        
    }

    printf("\nThe total number of increases is %d", totalIncrease);

    return 0;
}