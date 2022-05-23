create source ad_click (
    ad_id bigint,
    click_timestamp timestamp
) with (
    'connector' = 'kafka',
    'kafka.topic' = 'ad_click',
    'kafka.brokers' = 'redpanda:9092',
    'kafka.scan.startup.mode' = 'earliest'
) row format json;

create source ad_impression (
    bid_id bigint,
    ad_id bigint,
    impression_timestamp timestamp
) with (
    'connector' = 'kafka',
    'kafka.topic' = 'ad_impression',
    'kafka.brokers' = 'redpanda:9092',
    'kafka.scan.startup.mode' = 'earliest'
) row format json;

create materialized view ad_ctr as
select
    t1.ad_id as ad_id,
    t1.clicks_count :: numeric * 100 / t2.impressions_count as ctr
from
    (
        select
            ad_id,
            count(*) as clicks_count
        from
            ad_click
        group by
            ad_id
    ) as t1
    join (
        select
            ad_id,
            count(*) as impressions_count
        from
            ad_impression
        group by
            ad_id
    ) as t2 on t1.ad_id = t2.ad_id;