#include <stdio.h>
#include <math.h>

int main()
{
    int A, B, V;
    scanf("%d %d %d", &A, &B, &V);
    double n = (V - A) / (double)(A - B);
    int res = ceil(n) + 1;
    printf("%d\n", res);
    return 0;
}

// Compile the file as following
// gcc Q2869.c -lm
// Execute the file with the following command
// ./a.out