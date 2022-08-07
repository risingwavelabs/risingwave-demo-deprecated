CREATE MATERIALIZED SOURCE prometheus (
    labels STRUCT < __name__ VARCHAR,
    instance VARCHAR,
    job VARCHAR >,
    name VARCHAR,
    timestamp TIMESTAMP,
    value VARCHAR
) WITH (
    connector = 'kafka',
    kafka.topic = 'prometheus',
    kafka.brokers = '127.0.0.1:56582',
    kafka.scan.startup.mode = 'earliest'
) ROW FORMAT JSON;

-- 1. Create the source with Materialize.
CREATE SOURCE prometheus_metrics
FROM
    KAFKA BROKER '127.0.0.1:56582' TOPIC 'prometheus' FORMAT BYTES;

-- 2. Normalize the JSON.
CREATE MATERIALIZED VIEW metrics AS
SELECT
    data -> 'labels' AS labels,
    data ->> 'name' AS name,
    (data ->> 'timestamp') :: TIMESTAMP AS timestamp,
    (data ->> 'value') :: NUMERIC AS value
FROM
    (
        SELECT
            CONVERT_FROM(data, 'utf8') :: jsonb AS data
        FROM
            prometheus_metrics
    );

select
    *
from
    metrics
where
    name = 'object_store_read_bytes';

create materialized view metric_avg_15s as
select
    name as metric_name,
    window_start as metric_time,
    avg(value :: decimal) as metric_value
from
    tumble(
        prometheus,
        timestamp :: TIMESTAMP,
        interval '15 s'
    )
group by
    name,
    window_start;

create materialized view metric_avg_1min as
select
    metric_name,
    window_start as metric_time,
    avg(metric_value) as metric_value
from
    tumble(
        metric_avg_15s,
        metric_time,
        interval '1 min'
    )
group by
    metric_name,
    window_start;

create materialized view metric_avg_5min as
select
    metric_name,
    window_start as metric_time,
    avg(metric_value) as metric_value
from
    tumble(
        metric_avg_1min,
        metric_time,
        interval '5 min'
    )
group by
    metric_name,
    window_start;