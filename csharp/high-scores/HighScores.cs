using System;
using System.Collections.Generic;
using System.Linq;

public class HighScores
{
    private List<int> list;
    private int latest;
    private int best;
    private List<int> top;

    public HighScores(List<int> list)
    {
        this.list = list;
        latest = list.Last();
        top = this.list.OrderByDescending(x => x).Take(3).ToList();
        best = top.First();
    }

    public List<int> Scores()
    {
        return list;
    }

    public int Latest()
    {
        return latest;
    }

    public int PersonalBest()
    {
        return best;
    }

    public List<int> PersonalTopThree()
    {
        return top;
    }
}
