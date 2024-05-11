use std::error::Error;

use tonic::Request;
use tonic::transport::Channel;

use crate::proto;
use crate::proto::db_api_client::DbApiClient;
use crate::work::Work;

pub async fn add_work(
    client: &mut DbApiClient<Channel>,
    work: Work,
) -> tonic::Result<String, Box<dyn Error>> {
    let req = proto::ProtoWork {
        name: work.name,
        desc: work.desc,
        date_start: work.date_start,
        date_end: work.date_end,
    };

    let request = Request::new(req);

    let response = client.add_work(request).await?;

    dbg!(response.get_ref().clone());

    Ok(response.into_inner().index)
}

pub async fn get_work(
    client: &mut DbApiClient<Channel>,
    index: String,
) -> tonic::Result<Work, Box<dyn Error>> {
    let req = proto::ProtoWorkIndex { index };

    let request = Request::new(req);

    let response = client.get_work(request).await?;

    let resp = response.get_ref().clone();

    dbg!(resp.clone());
    let work = Work::from(resp.name, resp.desc, resp.date_start, resp.date_end);

    Ok(work)
}

pub async fn get_all_works(
    client: &mut DbApiClient<Channel>,
) -> tonic::Result<Vec<Work>, Box<dyn Error>> {
    let req = proto::Empty {};

    let request = Request::new(req);

    let response = client.get_all_works(request).await?;
    let mut works: Vec<Work> = Vec::new();

    for proto_work in response.get_ref().clone().works {
        works.push(Work::from(
            proto_work.name,
            proto_work.desc,
            proto_work.date_end,
            proto_work.date_start,
        ));
    }
    dbg!(response);

    Ok(works)
}
