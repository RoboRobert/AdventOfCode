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
                //If you need to lose, performs this block.
                case 'X':
                
                    switch(currentLineChars[0])
                    {
                        //If your opponent picked rock, you gain 3 points for picking scissors.
                        case 'A':
                            
                            totalScore = totalScore + 3;
                            break;
                        //If your opponent picked paper, you gain 1 point for picking rock.
                        case 'B':                    
                            totalScore = totalScore + 1;
                            break;

                        //If your opponent picked scissors, you gain 2 points for picking paper.
                        case 'C':
                            totalScore = totalScore + 2;
                            break;

                        default:
                            break;
                    }
                    break;

                //If you need to draw, performs this block.
                case 'Y':
                    //Gain 3 points for drawing.
                    totalScore = totalScore + 3;

                    switch(currentLineChars[0])
                    {
                        //If your opponent picked rock, you gain 1 point for picking rock.
                        case 'A':
                            totalScore = totalScore + 1;
                            break;
                        //If your opponent picked paper, you gain 2 points for picking paper.
                        case 'B':
                            totalScore = totalScore + 2;
                            break;
                        //If your opponent picked scissors, you gain 3 points for picking scissors.
                        case 'C':          
                            totalScore = totalScore + 3;   
                            break;

                        default:
                            break;
                    }
                    break;

                //If you need to win, performs this block.
                case 'Z':
                    //Gain 6 points for winning.
                    totalScore = totalScore + 6;

                    switch(currentLineChars[0])
                    {
                        //If your opponent picked rock, you gain 2 points for picking paper.
                        case 'A':
                            totalScore = totalScore + 2;
                            break;
                        //If your opponent picked paper, you gain 3 points for picking scissors.
                        case 'B':  
                            totalScore = totalScore + 3;
                            break;
                        //If your opponent picked scissors, you gain 1 point for picking rock.
                        case 'C':
                            totalScore = totalScore + 1;
                            break;

                        default:
                            break;
                    }
                    break;

                default:
                    break;
            }
        }
    }

    printf("\nThe total score is %d", totalScore);

    return 0;
}