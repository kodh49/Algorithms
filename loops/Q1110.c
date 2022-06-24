#include <stdio.h>

int cycle(int num)
{
    int dig10, dig1, exc;
    if (num < 10)
    {
        dig10 = 0;
        dig1 = num;
    }
    else
    {
        dig10 = num / 10;
        dig1 = num - dig10 * 10;
    }
    exc = dig1;
    if (dig10 + dig1 >= 10)
    {
        dig1 = (dig10 + dig1) - (10 * ((dig10 + dig1) / 10));
    }
    else
    {
        dig1 = dig10 + dig1;
    }
    dig10 = exc;
    num = 10 * dig10 + dig1;
    return num;
}

int main()
{
    int num, iter, count = 1;
    scanf("%d", &num);
    iter = num;
    while (num != cycle(iter))
    {
        iter = cycle(iter);
        count++;
    }
    printf("%d\n", count);
    return 0;
}