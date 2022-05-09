-- impression_timestamp: The time when the ad was shown.
-- click_timestamp: The time when the ad was clicked by the user.
create source ad_source (
    user_id bigint,
    ad_id bigint,
    click_timestamp timestamp,
    impression_timestamp timestamp
) with (
    'connector' = 'kafka',
    'kafka.topic' = 'test_topic',
    'kafka.brokers' = 'localhost:29092',
    'kafka.scan.startup.mode' = 'latest'
) row format json;

-- The number of users who clicked the ad within one minute after the ad was shown.
create materialized view m_click_statistic as
select
    count(user_id),
    ad_id
from
    ad_source
where
    click_timestamp is not null
    and impression_timestamp < click_timestamp
    and impression_timestamp + interval '1' minute >= click_timestamp
group by
    ad_id;