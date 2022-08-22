use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{Response, Status};
use crate::model::{RecallRequest, RecallResponse, GetRatingRequest, GetRatingResponse};
use crate::recommender::ReportActionResponse;
use crate::{ModelClient, Recwave};

impl Recwave {
    pub async fn recall(&self, userid: String) -> Result<Vec<String>, Status> {
        let request = RecallRequest {
            userid,
        };
        let mut model_client = ModelClient::connect("http://localhost:8080")
            .await
            .expect("Failed to connect to model server");
        let response = model_client.recall(request)
            .await;
        match response {
            Ok(resp) => {
                Ok(resp.into_inner().itemid)
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn sort(&self, userid: String, itemids: Vec<String>, max_pick: usize) -> Result<Vec<String>, Status> {
        let mut ratings = Vec::new();
        for itemid in itemids {
            let request = GetRatingRequest{
                userid: userid.clone(),
                itemid: itemid.clone(),
                feature_values: vec![0.0,1.0,0.0]
            };
            let mut model_client = ModelClient::connect("http://localhost:8080")
                .await
                .expect("Failed to connect to model server");
            let response = model_client
                .get_rating(request)
                .await?;
            let rating = response.into_inner().rating;
            ratings.push((itemid.clone(), rating));
        }
        ratings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().reverse());
        if ratings.len() > max_pick {
            ratings.truncate(max_pick);
        }
        let itemids = ratings.iter().map(|x| x.0.clone()).collect();

        Ok(itemids)
    }
}
