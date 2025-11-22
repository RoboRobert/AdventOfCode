#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>

int recursionRemoval(int array[1000][12], int currentIteration, int sort);

int recursionRemoval(int array[1000][12], int currentIteration, int sort)
{
    int remainingNumbers[2] = {0, 0};

    int iterationNum = currentIteration + 1;

    printf("\nCurrent iteration: %d", currentIteration);

    int totalNum0 = 0;
    int totalNum1 = 0;

    int newRemovalValue = 0;

    //Finds the number to remove at the current position in the array.
    for(int i = 0; i < 1000; i++)
    {
        
        if(array[i][0] != 2)
        {
            if(array[i][currentIteration] == 0)
                totalNum0 = totalNum0 + 1;

            else totalNum1 = totalNum1 + 1;

            remainingNumbers[0] = remainingNumbers[0] + 1;
        }
    }

    printf("\nNumbers remaining before removal: %d", remainingNumbers[0]);
    printf("\n# of 1s: %d",totalNum1);
    printf("\n# of 0s: %d",totalNum0);

    if(totalNum1 > totalNum0)
    {
        printf("\nMost Common: 1");
        printf("\nLeast Common: 0");
    }
    else if (totalNum0 > totalNum1)
    {
        printf("\nMost Common: 0");
        printf("\nLeast Common: 1");
    }
    else printf("\nEqual, num removed is %d", sort*1);

    //If the sort is one, then it removes entries with the value of the larger amount, and when the values are equal, defaults to removing 1
    if(sort == 1)
    {
        if(totalNum0 > totalNum1)
            newRemovalValue = 0;
        else if(totalNum1 > totalNum0)
            newRemovalValue = 1;
        else if(totalNum0 == totalNum1)
            newRemovalValue = 1;
    }
    //If the sort is 0, then it removes entries with the value of the smaller amount, and when the values are equal, defaults to removing 0
    else 
    {
        if(totalNum1 < totalNum0)
            newRemovalValue = 1;
        else if(totalNum0 < totalNum1)
            newRemovalValue = 0;
        else if(totalNum0 == totalNum1)
            newRemovalValue = 0;
    }
    
    //Removes all the unnecessary values from the array by setting their first element to 2.
    for(int i = 0; i < 1000; i++)
    {
        if(array[i][0] != 2 && array[i][currentIteration] == newRemovalValue) 
        {
            array[i][0] = 2;
            remainingNumbers[0]--;
        }
        if(array[i][0] != 2)
            remainingNumbers[1] = i + 1;
    }
    
    printf("\nNumbers remaining after removal: %d", remainingNumbers[0]);

    if(remainingNumbers[0] == 1)
        return remainingNumbers[1];

    recursionRemoval(array, iterationNum, sort);
}


int main()
{
    int i = 0;

    int location1 = 0;
    int location2 = 0;

    int finalGammaDec = 0;
    int finalEpsilonDec = 0;

    char fileContents[1000][20];

    int fileContentsInt[1000][12];

    char currentString[20];

    FILE *binaryInput;

    binaryInput = fopen("Day3Input.txt", "r");

    //Takes the values from the input file, converts them to ints, and stores them in an array.
    while(fgets(currentString, 20, binaryInput) != NULL)
    {
        strcpy(fileContents[i], currentString);


        i++;
    }

    for(i = 0; i < 1000; i++)
    {
        for(int j = 0; j < 12; j++)
        {
            fileContentsInt[i][j] = ((int)fileContents[i][j]) - 48;
            
        }
    }

    printf("\n%d", location1 = recursionRemoval(fileContentsInt, 0, 0));

    i = 0;

    //Takes the values from the input file, converts them to ints, and stores them in an array.
    while(fgets(currentString, 20, binaryInput) != NULL)
    {
        strcpy(fileContents[i], currentString);

        i++;
    }

    for(i = 0; i < 1000; i++)
    {
        for(int j = 0; j < 12; j++)
        {
            fileContentsInt[i][j] = ((int)fileContents[i][j]) - 48;
            
        }
    }

    printf("\n%d", location2 = recursionRemoval(fileContentsInt, 0, 1));

    //Takes the values from the input file, converts them to ints, and stores them in an array.
    while(fgets(currentString, 20, binaryInput) != NULL)
    {
        strcpy(fileContents[i], currentString);

        i++;
    }
    for(i = 0; i < 1000; i++)
    {
        for(int j = 0; j < 12; j++)
        {
            fileContentsInt[i][j] = ((int)fileContents[i][j]) - 48;
            
        }
    }

    fclose(binaryInput);

    printf("\nFinal Locations: %d, %d", location1, location2);

    //Prints the binary found at each location
    printf("\n"); 
    for(i = 0; i < 12; i++)
    {
        printf("%d", fileContentsInt[location1-1][i]);   
    }
    printf("\n"); 
    for(i = 0; i < 12; i++)
    {
        printf("%d", fileContentsInt[location2-1][i]);   
    }
    //Converts the elements at each location from binary to decimal
    for(i = 11; i >= 0; i--)
    {
        finalEpsilonDec = finalEpsilonDec + fileContentsInt[location2-1][i]*(pow(2, abs(i - 11)));
        finalGammaDec = finalGammaDec + fileContentsInt[location1-1][i]*(pow(2, abs(i - 11)));
    }

    printf("\nFinal Gamma: %d, Final Epsilon: %d, multiplied together: %d", finalGammaDec, finalEpsilonDec, finalGammaDec*finalEpsilonDec);

    return 0;
}