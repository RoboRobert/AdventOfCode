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
    int l = 0;

    int winningBoards[100] = {0};

    int totalBoards = 0;

    int tally = 0;
    int winningNum = 0;

    int boardNum = 0;

    int drawNumRem = 0;

    int winCondition = 0;

    int currentStrLen = 0;

    char currentString[500] = {' '};

    int numbersDrawnSize = 0;
    int numbersDrawn[500] = {0};

    int allBoards[700][5] = {0};

    FILE *bingoInput;

    bingoInput = fopen("Day4Input.txt", "r");

    //Reads in the first line to a string
    fgets(currentString, 500, bingoInput);

    //Sets the current string length in an integer.
    currentStrLen = strlen(currentString);

    //Converts the first line into an array of integers.
    for(i = currentStrLen - 2; i >=0; i--)
    {
        //Anytime a semicolon is encountered, increments j, resets k, then skips the rest of the loop.
        if(!isdigit(currentString[i]))
        {
            j++;
            numbersDrawnSize++;
            k = 0;
            continue;
        }
        //The current element in the array of ints has the current number in currentString added to it with the respective power(either 1 or 10 in this case)
        numbersDrawn[j] += ((int)(currentString[i]) - 48)*pow((int)10, k);
        //k is the current power that the integer is going to be raised to.
        k++;
    }
    // printf("%d", numbersDrawnSize);
    // printf("\n%d", numbersDrawn[numbersDrawnSize]);


    //Reads in all the boards and stores them in an array of integers. Empty lines are lines of zeros.
    //Increments fgets() by one to skip the empty line.
    // fgets(currentString, 30, bingoInput);
    while(fgets(currentString, 30, bingoInput) != NULL)
    {
        if(currentString[0] == '\n')
        {
            continue;
        }

        currentStrLen = strlen(currentString);

        //Converts each line into an array of 5 integers
        k = 0;
        j = 4;
        for(i = currentStrLen - 1; i >=0; i--)
        {
            //Anytime a non-digit input is encountered, resets k, then skips the rest of the loop.
            if(!isdigit(currentString[i]))
            {
                //If the space is encountered directly after a digit, decrements j.
                if(isdigit(currentString[i + 1]))
                    j--;
                k = 0;
                continue;
            }
            //The current element in the array of ints has the current number in currentString added to it with the respective power(either 1 or 10 in this case)
            allBoards[l][j] += ((int)(currentString[i]) - 48)*pow((int)10, k);
            
            //k is the current power that the integer is going to be raised to.
            k++;
        }
        l++;
    }

    //Calculates the total number of functioning boards.
    for(i = 0; i < 600; i = i + 5)
    {
        if(allBoards[i][0] == 0 && allBoards[i][1] == 0)
            break;
        totalBoards++;
    }
    printf("\n%d", totalBoards); 
    drawNumRem = numbersDrawnSize;
    while(winCondition < totalBoards)
    {
        printf("\nThe current number is %d", numbersDrawn[drawNumRem]);
        for(i = 0; i< 600; i++)
        {
            if(winningBoards[i/5] == 1)
                continue;
            if(allBoards[i][0] == 0 && allBoards[i][1] == 0)
                break;
                
            for(j = 0; j<5; j++)
            {
                if(allBoards[i][j]!= 500 && allBoards[i][j] == numbersDrawn[drawNumRem])
                {
                    allBoards[i][j] = 500;
                }
                // printf("%d ", allBoards[i][j]);
            }
            // printf("\n");
        }
        
        for(i = 0; i < 600; i = i + 5)
        {
            //If the current board has already won, skips this loop.
            if(winningBoards[i/5] == 1)
                continue;
            if(allBoards[i][0] == 0 && allBoards[i][1] == 0)
                break;
            for(j = 0; j < 5; j++)
            {
                if(allBoards[i + j][0] == 500 && allBoards[i + j][1] == 500 && allBoards[i + j][2] == 500 && allBoards[i + j][3] == 500 && allBoards[i + j][4] == 500)
                {
                    winCondition++;
                    winningBoards[i/5] = 1;
                    winningNum = numbersDrawn[drawNumRem];
                    boardNum = i;
                    break;
                }

                if(allBoards[i][j] == 500 && allBoards[i + 1][j] == 500 && allBoards[i + 2][j] == 500 && allBoards[i + 3][j] == 500 && allBoards[i + 4][j] == 500)
                {
                    winCondition++;
                    winningBoards[i/5] = 1;
                    winningNum = numbersDrawn[drawNumRem];
                    boardNum = i;
                    break;
                }
                if(winCondition == totalBoards)
                    break;
            }
            if(winCondition == totalBoards)
                break;
        }
        drawNumRem--;
    }
    
    printf("\nBoard: %d out of %d, Winning number: %d", boardNum/5, totalBoards, winningNum);
    printf("\n");
    for(i = 0; i < 5; i++)
    {
        for(j = 0; j < 5; j++)
        {
            if(allBoards[boardNum+i][j] != 500)
            {
                tally += allBoards[boardNum+i][j];
            }

            printf("%d ", allBoards[boardNum+i][j]);
        }
        printf("\n");
    }

    printf("\nThe final score is %d", tally*winningNum);
    
    fclose(bingoInput);

    return 0;
}