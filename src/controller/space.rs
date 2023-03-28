pub async fn create_space(
    store: crate::store::Store,
    new_space: crate::model::space::Space,
) -> Result<impl warp::Reply, warp::Rejection> {
    match create(store, new_space).await {
        Ok(space) => Ok(warp::reply::json(&space)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

async fn create(
    store: crate::store::Store,
    new_space: crate::model::space::Space,
) -> Result<crate::model::space::NewSpaceCreated, crate::error::Error> {
    match store.create_space(new_space).await {
        Ok(space) => Ok(crate::model::space::NewSpaceCreated {
            name: format!("{:?}", &space.name),
            uri: format!("/spaces/{:?}", &space.space_id.unwrap().0),
        }),
        Err(e) => Err(e),
    }
}
