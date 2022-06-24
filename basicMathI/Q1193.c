#include <stdio.h>

int main()
{
    int N;
    int min = 1;
    int max = 1;
    scanf("%d", &N);
    if (N == 1)
    {
        printf("1/1\n");
    }
    else
    {
        int k = 0;
        while (!(min <= N && max >= N))
        {
            k++;
            min += k;
            max += (k + 1);
        }
        int diagMove = N - min; // N - min => number of moves in diagonal line
        // sum = k + 2; We only know on which diagonal line the number is located
        // printf("Diagonal Move : %d\n", diagMove);
        // printf("Sum = k + 2 : %d\n", k + 2);
        if (k % 2 == 1)
        { // Going up
            printf("%d/%d\n", 1 + diagMove, k + 1 - diagMove);
        }
        else
        { // Going down
            printf("%d/%d\n", k + 1 - diagMove, 1 + diagMove);
        }
    }
    return 0;
}