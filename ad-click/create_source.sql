-- impression_timestamp: The time when the ad was shown.
-- click_timestamp: The time when the ad was clicked.
create source ad_source (
    user_id bigint,
    ad_id bigint,
    click_timestamp timestamptz,
    impression_timestamp timestamptz
) with (
    connector = 'kafka',
    kafka.topic = 'ad_clicks',
    kafka.brokers = 'message_queue:29092',
    kafka.scan.startup.mode = 'earliest'
) row format json;