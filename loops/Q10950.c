#include <stdio.h>

int main()
{
    int rep, a, b;
    scanf("%d", &rep);
    for (int i = 0; i < rep; i++)
    {
        scanf("%d %d", &a, &b);
        printf("%d\n", a + b);
    }
    return 0;
}