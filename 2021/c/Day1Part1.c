#include<stdio.h>
#include<stdlib.h>

int main()
{
    char currentLine[20];

    int previousNum = 0;

    int currentNum = 0;

    int totalIncrease = 0;

    FILE *sonarInput;

    sonarInput = fopen("Day1Input.txt", "r");

    //Increments fgets() once to skip the first line, since the first line
    fgets(currentLine, 20, sonarInput);
    previousNum = atoi(currentLine);

    while(fgets(currentLine, 20, sonarInput) != NULL)
    {
        currentNum = atoi(currentLine);

        if(currentNum > previousNum)
        {
            totalIncrease++;
        }
        previousNum = currentNum;
    }

    printf("\nThe total number of increases is %d", totalIncrease);

    return 0;
}