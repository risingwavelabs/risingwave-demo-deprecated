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
    ad_id bigint,
    impression_timestamp timestamp
) with (
    'connector' = 'kafka',
    'kafka.topic' = 'ad_impression',
    'kafka.brokers' = 'redpanda:9092',
    'kafka.scan.startup.mode' = 'earliest'
) row format json;

create materialized view mv_ctr_per_client as
select
    impression_ad as ad_id,
    impression_client,
    count(distinct impression_timestamp) as impression_ad_count,
    count(distinct click_ad) as click_ad_count,
    count(distinct click_ad) :: numeric / count(distinct impression_timestamp) :: numeric as ctr_per_client
from
    (
        select
            ad_impression.ad_id as impression_ad,
            ad_impression.client_id as impression_client,
            ad_click.client_id as click_client,
            ad_click.ad_id as click_ad,
            ad_click.user_name as click_user,
            ad_impression.impression_timestamp,
            ad_click.click_timestamp
        from
            ad_impression
            left join ad_click on ad_impression.ad_id = ad_click.ad_id
        where
            ad_impression.impression_timestamp <= ad_click.click_timestamp - interval '1 seconds'
    ) as join_table
group by
    impression_ad,
    impression_client;

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