#if !defined(SPACE_AGE_H)
#define SPACE_AGE_H

namespace space_age {

class space_age {
    public:
        space_age(unsigned long);
        ~space_age();

        unsigned long seconds() const;
        double on_earth() const;
        double on_mercury() const;
        double on_venus() const;
        double on_mars() const;
        double on_jupiter() const;
        double on_saturn() const;
        double on_uranus() const;
        double on_neptune() const;

   private:
        unsigned long age;
        static constexpr unsigned long oy_s = 31557600;

        double in_earth_years(double orbital_period) const;
};

space_age::space_age(unsigned long seconds)
{
    age = seconds;
}

space_age::~space_age()
{
}

inline double space_age::in_earth_years(double orbital_period) const
{
    return (double(age) / double(oy_s)) / orbital_period;
}

inline unsigned long space_age::seconds() const
{
    return age;
}

inline double space_age::on_earth() const
{
    return in_earth_years(1.0);
}

inline double space_age::on_mercury() const
{
    return in_earth_years(0.2408467);
}

inline double space_age::on_venus() const
{
    return in_earth_years(0.61519726);
}

inline double space_age::on_mars() const
{
    return in_earth_years(1.8808158);
}

inline double space_age::on_jupiter() const
{
    return in_earth_years(11.862615);
}

inline double space_age::on_saturn() const
{
    return in_earth_years(29.447498);
}

inline double space_age::on_uranus() const
{
    return in_earth_years(84.016846);
}

inline double space_age::on_neptune() const
{
    return in_earth_years(164.79132);
}

}
#endif
