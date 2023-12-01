#include<stdio.h>
#include<stdlib.h>

int main(void)
{
    int currentLine = 0;

    int lineMarker = 1;

    int currentNum = 0;

    int maxCaloriesComparison = 0;

    int currentMaxCalories = 0;

    char currentString[20];

    FILE *elfCaloriesInput;

    elfCaloriesInput = fopen("C:\\Users\\Nathan\\Documents\\AdventOfCode\\DayOneFiles\\DayOneInput.txt", "r");

    while(!feof(elfCaloriesInput))
    {
        currentLine++;

        fgets(currentString, 20, elfCaloriesInput);

        currentNum = atoi(currentString);

        if(currentString[0] == '\n')
        {
            if(maxCaloriesComparison > currentMaxCalories)
            {
                currentMaxCalories = maxCaloriesComparison;

                lineMarker = currentLine;
            }
                
            maxCaloriesComparison = 0;
        }
        else
        {
            currentNum = atoi(currentString);
            maxCaloriesComparison = maxCaloriesComparison + currentNum;
        }
    }

    fclose(elfCaloriesInput);

    printf("\nThe largest number of calories is %d, found right before line %d", currentMaxCalories, lineMarker);

    return 0;
}