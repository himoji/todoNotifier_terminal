use std::error::Error;

use tonic::Request;
use tonic::transport::Channel;

use crate::proto;
use crate::proto::db_api_client::DbApiClient;
use crate::work::{Work, WorkParams};

pub async fn proto_add_work(
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

pub async fn proto_get_work(
    client: &mut DbApiClient<Channel>,
    index: String,
) -> tonic::Result<Work, Box<dyn Error>> {
    let i = index.clone();
    let req = proto::ProtoWorkIndex { index };

    let request = Request::new(req);

    let response = client.get_work(request).await?;

    let resp = response.get_ref().clone();

    dbg!(resp.clone());
    let work = Work::from(resp.name, resp.desc, resp.date_start, resp.date_end, i);

    Ok(work)
}

pub async fn proto_get_all_works(
    client: &mut DbApiClient<Channel>,
) -> tonic::Result<Vec<Work>, Box<dyn Error>> {
    let req = proto::Empty {};

    let request = Request::new(req);

    let response = client.get_all_works(request).await?;
    let mut works: Vec<Work> = Vec::new();

    for proto_work in response.get_ref().clone().works {
        dbg!(proto_work.clone());

        works.push(Work::from(
            proto_work.name,
            proto_work.desc,
            proto_work.date_start,
            proto_work.date_end,
            proto_work.index,
        ));
    }
    dbg!(response);

    Ok(works)
}

pub async fn proto_edit_work(
    client: &mut DbApiClient<Channel>,
    index: String,
    param: WorkParams,
) -> tonic::Result<Work, Box<dyn Error>> {
    let enums = match param {
        WorkParams::Name(name) => proto::ProtoWorkParam {
            index: index.clone(),
            r#enum: 0,
            value: name,
        },
        WorkParams::Desc(desc) => proto::ProtoWorkParam {
            index: index.clone(),
            r#enum: 1,
            value: desc,
        },
        WorkParams::DateStart(date_start) => proto::ProtoWorkParam {
            index: index.clone(),
            r#enum: 2,
            value: date_start.to_string(),
        },
        WorkParams::DateEnd(date_end) => proto::ProtoWorkParam {
            index: index.clone(),
            r#enum: 3,
            value: date_end.to_string(),
        },
    };

    let request = Request::new(enums);

    let response = client.edit_work(request).await?;
    let resp = response.get_ref().clone();

    dbg!(&resp);

    Ok(Work::from(
        resp.name,
        resp.desc,
        resp.date_start,
        resp.date_end,
        index.clone(),
    ))
}
