#include<stdio.h>
#include<stdlib.h>

int main(void)
{
    int currentNum = 0;

    int maxCaloriesComparison = 0;

    int tempStorage = 0;

    int number2Calories = 0;

    int number3Calories = 0;

    int currentMaxCalories = 0;

    char currentString[20];

    FILE *elfCaloriesInput;

    //Opens the file.
    elfCaloriesInput = fopen("C:\\Users\\Nathan\\Documents\\AdventOfCode\\DayOneFiles\\DayOneInput.txt", "r");

    //This loop ends at the end of the file.
    while(!feof(elfCaloriesInput))
    {

        //Gets the current line of the string and increments the line counter.
        fgets(currentString, 20, elfCaloriesInput);

        //Everything in this block happens when the current line is empty. All the data collected in the most recent block of integers is now put to use.
        if(currentString[0] == '\n')
        {
            //If the value of the most recent block of integers is larger than the largest value, it shifts the top three values down by one and sets the largest as the most recently found.
            if(maxCaloriesComparison > currentMaxCalories)
            {
                tempStorage = number2Calories;
                number2Calories = currentMaxCalories;

                number3Calories = tempStorage;

                currentMaxCalories = maxCaloriesComparison;
                
            }

            //If the value of the most recent block of integers is only larger than the second largest value, it shifts the second largest value into 3rd place and replaces it with the
            //value most recently found.
            else if(maxCaloriesComparison > number2Calories)
            {
                number3Calories = number2Calories;

                number2Calories = maxCaloriesComparison;
            }

            //If the value of the most recent block of integers is only larger than the third largest value, it replace the third largest with the most recently found value.
            else if(maxCaloriesComparison > number3Calories)
            {
                number3Calories = maxCaloriesComparison;
            }
            
            //After all these calculations, resets the comparison value to zero to prepare for the next chunk of integers.
            maxCaloriesComparison = 0;
        }

        //If the current line isn't an empty line, it adds the current integer value to the comparison value.
        else
        {
            currentNum = atoi(currentString);
            maxCaloriesComparison = maxCaloriesComparison + currentNum;
        }
    }

    //Closes the file.
    fclose(elfCaloriesInput);

    //Prints the top 3 values and their sum.
    printf("\n#1 Calories: %d, #2 Calories: %d, #3 Calories: %d, Sum: %d", currentMaxCalories, number2Calories, number3Calories, currentMaxCalories + number2Calories + number3Calories);

    return 0;
}