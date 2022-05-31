-- impression_timestamp: The time when the ad was shown.
-- click_timestamp: The time when the ad was clicked.
create source ad_source (
    user_id bigint,
    ad_id bigint,
    click_timestamp timestamp,
    impression_timestamp timestamp
) with (
    'connector' = 'kafka',
    'kafka.topic' = 'ad_clicks',
    'kafka.brokers' = 'message_queue:9092',
    'kafka.scan.startup.mode' = 'earliest'
) row format json;

-- The number of clicks on the ad within one minute after the ad was shown.
create materialized view m_click_statistic as
select
    count(user_id) as clicks_count,
    ad_id
from
    ad_source
where
    click_timestamp is not null
    and impression_timestamp < click_timestamp
    and impression_timestamp + interval '1' minute >= click_timestamp
group by
    ad_id;