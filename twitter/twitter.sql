CREATE SOURCE twitter (
    data STRUCT < created_at TIMESTAMP,
    id VARCHAR,
    text VARCHAR,
    lang VARCHAR >,
    author STRUCT < created_at TIMESTAMP,
    id VARCHAR,
    name VARCHAR,
    username VARCHAR >,
) WITH (
    'connector' = 'kafka',
    'kafka.topic' = 'twitter',
    'kafka.brokers' = 'message_queue:29092',
    'kafka.scan.startup.mode' = 'earliest'
) ROW FORMAT JSON;

CREATE MATERIALIZED VIEW hot_hashtags AS WITH tags AS (
    SELECT
        unnest(regexp_matches((data).text, '#\w+', 'g')) AS hashtag,
        (data).created_at as created_at
    FROM
        twitter
)
SELECT
    hashtag,
    COUNT(*) as hashtag_occurrences
FROM
    tags
WHERE
    created_at::Date = CURRENT_DATE
GROUP BY
    hashtag;

--
-- Postgres
--
CREATE TYPE twitter_user AS (
    created_at TIMESTAMP,
    id VARCHAR,
    name VARCHAR,
    username VARCHAR
);

CREATE TYPE tweet_data AS (
    created_at TIMESTAMP,
    id VARCHAR,
    text VARCHAR,
    lang VARCHAR
);

CREATE TABLE twitter (data tweet_data, author twitter_user);