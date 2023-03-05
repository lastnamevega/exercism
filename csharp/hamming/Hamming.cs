using System;

public static class Hamming
{
    public static int Distance(string firstStrand, string secondStrand)
    {
        var length = firstStrand.Length;

        if (length != secondStrand.Length)
            throw new ArgumentException();

        var distance = 0;

        for (int i = 0; i < length; i++)
        {
            if (firstStrand[i] != secondStrand[i])
                distance++;
        }

        return distance;
    }
}
