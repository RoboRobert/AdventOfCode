#include<stdio.h>
#include<stdlib.h>
#include<string.h>

int main()
{
    //Counter variables
    int i = 0;
    int j = 0;
    int k = 0;

    int totalPriority = 0;

    char currentString1[50];
    char currentString2[50];
    char currentString3[50];

    int currentStringLength1;
    int currentStringLength2;
    int currentStringLength3;

    char currentDupeChar;

    int currentDupeCharNum = 0;

    FILE *rucksackInput;

    //Opens the file
    rucksackInput = fopen("DayThreeInput.txt", "r");

    while(fgets(currentString1, 50, rucksackInput) != NULL)
    {
        fgets(currentString2, 50, rucksackInput);
        fgets(currentString3, 50, rucksackInput);

        //Makes sure the current line isn't empty
        if(currentString1[0] == '\n')
        {
            continue;
        }

        currentStringLength1 = strlen(currentString1);
        currentStringLength2 = strlen(currentString2);
        currentStringLength3 = strlen(currentString3);

        //Loops through the first half of the string and compares each value to each value in the second half.
        for(int i = 0; i <currentStringLength1; i++)
        {
            for(int j = 0; j <currentStringLength2; j ++)
            {
                if(currentString1[i] == currentString2[j])
                {
                    for(k = 0; k < currentStringLength3; k++)
                    {
                        if(currentString1[i] == currentString3[k])
                        {
                            currentDupeChar = currentString1[i];

                            currentDupeCharNum = currentDupeChar;

                            if(currentDupeCharNum >= 97)
                            {
                                currentDupeCharNum = currentDupeCharNum - 96;
                            }
                            else currentDupeCharNum = currentDupeCharNum - 38;

                            i = 100;
                            j = 100;
                            k = 100;
                        }
                        
                    }
                }
            }
        }

        totalPriority = totalPriority + currentDupeCharNum;
    }
    printf("\nThe added priorities of all the shared letters in the groups of three is %d", totalPriority);

    return 0;
}