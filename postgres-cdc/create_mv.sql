CREATE MATERIALIZED VIEW city_population AS
SELECT
    city,
    COUNT(*) as population
FROM
    person
GROUP BY
    city;