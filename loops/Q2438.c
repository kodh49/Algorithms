#include <stdio.h>

int main()
{
    int rep;
    scanf("%d", &rep);
    for (int i = 1; i <= rep; i++)
    {
        for (int j = 0; j < i; j++)
        {
            printf("*");
        }
        printf("\n");
    }
    return 0;
}