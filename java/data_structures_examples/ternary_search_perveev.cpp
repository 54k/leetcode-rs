double f(double x, double y)
{
    return 2 * x * x + y * y / 5 + 2 * x;
}

double findY(double x)
{
    double l = -100, r = 100;
    for (int it = 0; it < 100; ++it)
    {
        double m1 = (2 * l + r) / 3.0; // l + (r - l) / 3.0;
        double m2 = (l + 2 * r) / 3.0; // r - (r - l) / 3.0;

        if (f(x, m1) < f(x, m2))
        {
            l = m1;
        }
        else
        {
            r = m2;
        }
    }
    return (l + r) / 2;
}

double findX()
{
    double l = -100, r = 100;
    for (int it = 0; it < 100; ++it)
    {
        double m1 = (2 * l + r) / 3.0; // l + (r - l) / 3.0;
        double m2 = (l + 2 * r) / 3.0; // r - (r - l) / 3.0;

        double y1 = findY(m1);
        double y2 = findY(m2);
        if (f(m1, y1) < f(m2, y2))
        {
            l = m1;
        }
        else
        {
            r = m2;
        }
    }
    return (l + r) / 2.0;
}