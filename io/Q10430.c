#include <stdio.h>
#include <math.h>

int main()
{
    int A, B, C;
    scanf("%d %d %d", &A, &B, &C);
    // 1st line
    int result = fmod((A + B), C);
    printf("%d\n", result);
    // 2nd line
    result = fmod(fmod(A, C) + fmod(B, C), C);
    printf("%d\n", result);
    // 3rd line
    result = fmod((A * B), C);
    printf("%d\n", result);
    // 4th line
    result = fmod(fmod(A, C) * fmod(B, C), C);
    printf("%d\n", result);
    return 0;
}

// 첫째 줄에 (A+B)%C, 둘째 줄에 ((A%C) + (B%C))%C, 셋째 줄에 (A×B)%C, 넷째 줄에 ((A%C) × (B%C))%C를 출력한다.