#include <stdio.h>

int main()
{
    int T, A, B, result;
    scanf("%d", &T);
    for (int i = 0; i < T; i++)
    {
        scanf("%d %d", &A, &B);
        result = A + B;
        printf("%d\n", result);
    }
}