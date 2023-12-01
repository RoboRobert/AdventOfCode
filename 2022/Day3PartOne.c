#include<stdio.h>
#include<stdlib.h>
#include<string.h>

int main()
{
    //Counter variables
    int i = 0;
    int j = 0;

    int totalPriority = 0;

    char currentString[50];

    int currentStringLength;

    char currentDupeChar;

    int currentDupeCharNum = 0;

    FILE *rucksackInput;

    //Opens the file
    rucksackInput = fopen("C:\\Users\\Nathan\\Documents\\AdventOfCode\\Day3Files\\DayThreeInput.txt", "r");

    //Reads the current line in the file, stores it in a 50 length array, and stops reading when fgets() returns null.
    while(fgets(currentString, 50, rucksackInput) != NULL)
    {
        //Read the current string and stores it in a 50 length array.        

        //Makes sure the current line isn't empty
        if(currentString[0] == '\n')
        {
            continue;
        }

        currentStringLength = strlen(currentString);

        //Loops through the first half of the string and compares each value to each value in the second half.
        for(int i = 0; i <currentStringLength/2; i++)
        {
            for(int j = currentStringLength/2; j < currentStringLength; j++)
            {
                //If it finds a match, it adds the priority of the value found to the total and breaks the loop.
                if(currentString[i] == currentString[j])
                {
                    currentDupeChar = currentString[i];

                    currentDupeCharNum = currentDupeChar;

                    if(currentDupeCharNum >= 97)
                    {
                        currentDupeCharNum = currentDupeCharNum - 96;
                    }
                    else currentDupeCharNum = currentDupeCharNum - 38;

                    //Sets the counter variables to a higher value than the max of the for() loops.
                    i = 100;
                    j = 100;
                }
            }
        
        }

        totalPriority = totalPriority + currentDupeCharNum;
    }
    
    printf("\nAll the priorities of the duplicated numbers totaled is: %d", totalPriority);

    return 0;
}