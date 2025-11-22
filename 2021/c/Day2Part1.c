#include<stdio.h>
#include<stdlib.h>
#include<string.h>

int main()
{
    char currentLine[20];

    char direction;

    int distanceInt = 0;

    int totalHorizontal = 0;

    int totalVertical = 0;

    FILE *subInput;

    subInput = fopen("Day2Input.txt", "r");

    while(fgets(currentLine, 20, subInput) != NULL)
    {
        direction = currentLine[0];

        distanceInt = (int)(currentLine[strlen(currentLine) -2]) - 48;

        if(direction == 'u')
            totalVertical = totalVertical - distanceInt;
        if(direction == 'd')
            totalVertical = totalVertical + distanceInt;
        if(direction == 'f')
            totalHorizontal = totalHorizontal + distanceInt;
    }

    printf("\nThe total values are H:%d, V:%d. Multiplied together: %d", totalHorizontal, totalVertical, totalHorizontal*totalVertical);

    return 0;
}