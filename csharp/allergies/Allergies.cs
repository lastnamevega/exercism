using System;
using System.Collections.Generic;

public enum Allergen
{
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128
}

public class Allergies
{
    private int score;

    public Allergies(int mask)
    {
        score = mask;
    }

    public bool IsAllergicTo(Allergen allergen)
    {
        return (score & (int)allergen) > 0;
    }

    public Allergen[] List()
    {
        List<Allergen> allergens = new List<Allergen>();

        foreach (Allergen a in Enum.GetValues(typeof(Allergen)))
        {
            if (IsAllergicTo(a))
            {
                allergens.Add(a);
            }
        }

        return allergens.ToArray();
    }
}
