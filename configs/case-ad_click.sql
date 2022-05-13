create materialized view mv_ctr_per_client as
select
    impression_ad as ad_id,
    impression_client,
    count(distinct impression_timestamp) as impression_ad_count,
    count(distinct click_ad) as click_ad_count,
    (cast (count(distinct click_ad) as numeric)) / (
        cast (count(distinct impression_timestamp) as numeric)
    ) as ctr_per_client
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