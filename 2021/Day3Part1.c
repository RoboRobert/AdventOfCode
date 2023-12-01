#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>

int main()
{
    char currentString[20];

    int currentStringLength = 0;

    int currentNum = 0;

    int totalNum0[12] = {0};
    long totalNum1[12] = {0};

    int finalGamma[12] = {0};
    long finalEpsilon[12] = {0};

    int finalGammaDec = {0};
    int finalEpsilonDec = {0};

    FILE *binaryInput;

    binaryInput = fopen("Day3Input.txt", "r");

    while(fgets(currentString, 20, binaryInput) != NULL)
    {
        currentStringLength = strlen(currentString);
       
        for(int i = 0; i < currentStringLength; i++)
        {
            currentNum = ((int)currentString[i]) - 48;
            if(currentNum == 0)
            {
                totalNum0[i] = totalNum0[i] + 1;
            }
            else totalNum1[i] = totalNum1[i] + 1;
            
        }
    }

    for(int i = 0; i < 12; i++)
    {
        if(totalNum0[i] > totalNum1[i])
        {
            finalGamma[i] = 0;
            finalEpsilon[i] = 1;
        }
        else
        {
            finalGamma[i] = 1;
            finalEpsilon[i] = 0;
        }
    }

    // printf("\nThe total number of zeros in each column is: ");
    // for(int i = 0; i < 12; i++)
    // {
    //     printf("%d, ", totalNum0[i]);
    // }

    // printf("\nThe total number of ones in each column is: ");
    // for(int i = 0; i < 12; i++)
    // {
    //     printf("%d, ", totalNum1[i]);
    // }
    
    //The final binary value for Gamma
    printf("\nFinal Gamma Binary: ");
    for(int i = 0; i < 12; i++)
    {
        printf("%d", finalGamma[i]);
    }
    //The final decimal value for Gamma
    for(int i = 11; i >= 0 ; i--)
    {
        finalGammaDec = finalGammaDec + finalGamma[i]*((int)pow(2, abs((i) - 11)));
    }
    printf("\nFinal Gamma Decimal: %d", finalGammaDec);
    

    //The final Binary value for Epsilon
    printf("\nFinal Epsilon Binary: ");
    for(int i = 0; i < 12; i++)
    {
        printf("%ld", finalEpsilon[i]);
    }
    //The final decimal value for Epsilon 
    for(int i = 11; i >= 0 ; i--)
    {
        finalEpsilonDec = finalEpsilonDec + finalEpsilon[i]*(pow(2, abs(i - 11)));
    }
    printf("\nFinal Epsilon Decimal: %d", finalEpsilonDec);

    printf("\nMultiplied together: %d", finalEpsilonDec * finalGammaDec);

    fclose(binaryInput);
    
    return 0;
}