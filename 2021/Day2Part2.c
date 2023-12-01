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

    int aim = 0;

    FILE *subInput;

    subInput = fopen("Day2Input.txt", "r");

    while(fgets(currentLine, 20, subInput) != NULL)
    {
        direction = currentLine[0];

        distanceInt = (int)(currentLine[strlen(currentLine) -2]) - 48;

        if(direction == 'u')
            aim = aim - distanceInt;
        if(direction == 'd')
            aim = aim + distanceInt;
        if(direction == 'f')
        {
            totalHorizontal = totalHorizontal + distanceInt;

            totalVertical = totalVertical + aim*distanceInt;
        }

    }

    printf("\nFinal V: %d, Final H: %d, Multiplied together: %d",totalVertical, totalHorizontal, totalVertical*totalHorizontal);

    return 0;
}