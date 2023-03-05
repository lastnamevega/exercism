using System;

public static class Gigasecond
{
    public static readonly double GIGASECOND = 1E9;

    public static DateTime Add(DateTime moment)
    {
        return moment.AddSeconds(GIGASECOND);
    }
}
