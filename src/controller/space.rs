use regex::Regex;

pub async fn create_space(
    store: crate::store::Store,
    new_space: crate::model::space::Space,
) -> Result<impl warp::Reply, warp::Rejection> {
    if new_space.name.chars().count() > 255 {
        return Err(warp::reject::custom(
            crate::error::Error::IllegalArgumentException(String::from("Space name too long")),
        ));
    }
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]{1,29}$").unwrap();
    println!(
        "REGEXP for '{}' is {}",
        &new_space.owner,
        re.is_match(&new_space.owner)
    );
    if !re.is_match(&new_space.owner) {
        return Err(warp::reject::custom(
            crate::error::Error::IllegalArgumentException(format!(
                "Invalid username: '{}'",
                &new_space.owner
            )),
        ));
    }
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
            name: format!("{}", &space.name),
            uri: format!("/spaces/{}", &space.space_id.unwrap().0),
        }),
        Err(e) => Err(e),
    }
}
