#include <stdio.h>

int main()
{
    int a, b;
    scanf("%d %d", &a, &b);
    int sum = a + b;
    int sub = a - b;
    int mul = a * b;
    int quot = a / b;
    int remain = a % b;
    printf("%d\n", sum);
    printf("%d\n", sub);
    printf("%d\n", mul);
    printf("%d\n", quot);
    printf("%d\n", remain);
    return 0;
}

// + - * / % 순서대로 작성