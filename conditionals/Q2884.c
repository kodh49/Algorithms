#include <stdio.h>

int main(void)
{
    int hour, minute;
    scanf("%d %d", &hour, &minute);
    if (minute >= 45)
    {
        printf("%d %d", hour, minute - 45);
    }
    else
    {
        if (hour < 1)
        {
            hour = 23;
        }
        else
        {
            --hour;
        }
        minute = 15 + minute;
        printf("%d %d", hour, minute);
    }
    return 0;
}

// 45 minutes earlier than the time set