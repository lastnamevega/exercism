using System;
using System.Collections.Generic;

public class GradeSchool
{
    private List<Tuple<int, string>> Students;

    public GradeSchool()
    {
        Students = new List<Tuple<int, string>>();
    }

    public void Add(string student, int grade)
    {
        Students.Add(new Tuple<int, string>(grade, student));
        Students.Sort();
    }

    public IEnumerable<string> Roster()
    {
        return Students.ConvertAll(student => student.Item2);
    }

    public IEnumerable<string> Grade(int grade)
    {
        Predicate<Tuple<int, string>> Match = Student => Student.Item1 == grade;
        List<Tuple<int, string>> Matching = Students.FindAll(Match);
        List<string> Names = Matching.ConvertAll(Student => Student.Item2);

        return Names;
    }
}
