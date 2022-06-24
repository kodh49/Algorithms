#include <stdio.h>

int main()
{
    int N;
    int min = 2;
    int max = 7;
    scanf("%d", &N);
    if (N == 1)
    {
        printf("1\n");
    }
    else
    {
        int k = 0;
        while (!(min <= N && max >= N))
        {
            k++;
            min += 6 * k;
            max += 6 * (k + 1);
        }
        printf("%d\n", k + 2);
    }
    return 0;
}