#include <stdio.h>

int main()
{
    int xcord, ycord;
    scanf("%d", &xcord);
    scanf("%d", &ycord);
    if (xcord > 0)
    {
        if (ycord > 0)
        {
            printf("1");
        }
        else
        {
            printf("4");
        }
    }
    else
    {
        if (ycord > 0)
        {
            printf("2");
        }
        else
        {
            printf("3");
        }
    }
    return 0;
}