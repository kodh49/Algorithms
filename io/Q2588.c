#include <stdio.h>

int main()
{
    int num1, num2, result;
    scanf("%d\n", &num1);
    scanf("%d", &num2);
    int arr[3];
    arr[2] = num2 / 100;
    arr[1] = (num2 - (arr[2] * 100)) / 10;
    arr[0] = (num2 - (arr[2] * 100) - (arr[1] * 10));
    for (int i = 0; i < 3; i++)
    {
        printf("%d\n", num1 * arr[i]);
    }
    printf("%d", num1 * num2);
    return 0;
}