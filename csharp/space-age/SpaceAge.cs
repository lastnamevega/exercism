using System;

public class SpaceAge
{
    private double EarthAge;
    private const double EARTH_YEAR_SECONDS = 31557600;
    private const double MERCURY_DIVISOR = 0.2408467;
    private const double VENUS_DIVISOR = 0.61519726;
    private const double MARS_DIVISOR = 1.8808158;
    private const double JUPITER_DIVISOR = 11.862615;
    private const double SATURN_DIVISOR = 29.447498;
    private const double URANUS_DIVISOR = 84.016846;
    private const double NEPTUNE_DIVISOR = 164.79132;

    public SpaceAge(int seconds)
    {
        this.EarthAge = seconds / EARTH_YEAR_SECONDS;
    }

    public double OnEarth()
    {
        return this.EarthAge;
    }

    public double OnMercury()
    {
        return this.EarthAge / MERCURY_DIVISOR;
    }

    public double OnVenus()
    {
        return this.EarthAge / VENUS_DIVISOR;
    }

    public double OnMars()
    {
        return this.EarthAge / MARS_DIVISOR;
    }

    public double OnJupiter()
    {
        return this.EarthAge / JUPITER_DIVISOR;
    }

    public double OnSaturn()
    {
        return this.EarthAge / SATURN_DIVISOR;
    }

    public double OnUranus()
    {
        return this.EarthAge / URANUS_DIVISOR;
    }

    public double OnNeptune()
    {
        return this.EarthAge / NEPTUNE_DIVISOR;
    }
}
