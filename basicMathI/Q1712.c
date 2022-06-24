#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

int main()
{
    int A, B, C;
    scanf("%d %d %d", &A, &B, &C);
    if (C - B > 0)
    {
        int n = A / (C - B);
        printf("%d\n", n + 1);
    }
    else
    {
        printf("-1\n");
    }
    return 0;
}