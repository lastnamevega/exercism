public static class Leap
{
    private static bool IsDivisibleBy(int number, int divisor)
    {
        return number % divisor == 0;
    }

    public static bool IsLeapYear(int year)
    {
        return IsDivisibleBy(year, 400) ||
            (IsDivisibleBy(year, 4) && !IsDivisibleBy(year, 100));
    }
}
