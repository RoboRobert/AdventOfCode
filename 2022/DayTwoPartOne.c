#include<stdio.h>
#include<stdlib.h>

int main(void)
{
    char currentLineChars[20];

    int totalScore = 0;

    FILE *rpsInput;

    //Opens the file
    rpsInput = fopen("C:\\Users\\Nathan\\Documents\\AdventOfCode\\DayTwoFiles\\DayTwoInput.txt", "r");

    while(!feof(rpsInput))
    {
        fgets(currentLineChars, 20, rpsInput);

    
        if(currentLineChars[0] != '\n')
        {
            switch(currentLineChars[2])
            {
                //If you picked rock, it performs this block
                case 'X':
                    //Gives you 1 point for picking Rock
                    totalScore = totalScore + 1;
                    switch(currentLineChars[0])
                    {
                        //If your opponent picked rock, it's a draw and you gain 3 points.
                        case 'A':
                            
                            totalScore = totalScore + 3;
                            break;
                        //If your opponent picked paper, you lose and gain no points.
                        case 'B':                    
                            break;

                        //If your opponent picked scissors, you win and gain 6 points.
                        case 'C':
                            totalScore = totalScore + 6;
                            break;

                        default:
                            break;
                    }
                    // printf("\nYou picked Rock");
                    break;

                //If you picked paper, it performs this block
                case 'Y':
                    //Gives you 2 points for picking Paper
                    totalScore = totalScore + 2;
                    switch(currentLineChars[0])
                    {
                        //If your opponent picked rock, you win and gain 6 points.
                        case 'A':
                            totalScore = totalScore + 6;
                            break;
                        //If your opponent picked paper, it's a draw and you gain 3 points.
                        case 'B':
                            totalScore = totalScore + 3;
                            break;
                        //If your opponent picked scissors, you lose and gain no points.
                        case 'C':             
                            break;

                        default:
                            break;
                    }
                    // printf("\nYou picked Paper");
                    break;

                //If you picked scissors, it performs this block
                case 'Z':
                    //Gives you 3 points for picking Scissors
                    totalScore = totalScore + 3;

                    switch(currentLineChars[0])
                    {
                        //If your opponent picked rock, you lose and gain no points.
                        case 'A':
                            break;
                        //If your opponent picked paper, you win and gain 6 points.
                        case 'B':  
                            totalScore = totalScore + 6;
                            break;
                        //If your opponent picked scissors, it's a draw and you gain 3 points.
                        case 'C':
                            totalScore = totalScore + 3;
                            break;

                        default:
                            break;
                    }
                    // printf("\nYou picked Scissors");
                    break;
                default:
                    break;
            }
        }
    }

    printf("\nThe total score is %d", totalScore);

    return 0;
}