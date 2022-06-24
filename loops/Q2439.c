#include <stdio.h>

int main()
{
    int rep;
    scanf("%d", &rep);
    for (int i = 1; i <= rep; i++)
    {
        int j;
        for (j = 0; j < rep - i; j++)
        {
            printf(" ");
        }
        while (j < rep)
        {
            printf("*");
            j++;
        }
        printf("\n");
    }
    return 0;
}