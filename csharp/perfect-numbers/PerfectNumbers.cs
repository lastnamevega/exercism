using System;
using System.Linq;

public enum Classification
{
    Perfect,
    Abundant,
    Deficient
}

public static class PerfectNumbers
{
    private static int SumFactors(int number)
    {
        var sum = 0;

        for (int i = 1; i <= number / 2; i++)
        {
            if (number % i == 0)
                sum += i;
        }

        return sum;
    }

    public static Classification Classify(int number)
    {
        if (number < 1)
            throw new ArgumentOutOfRangeException();

        var sum = SumFactors(number);

        if (sum < number)
            return Classification.Deficient;

        if (sum > number)
            return Classification.Abundant;

        return Classification.Perfect;
    }
}
