use ort::session::Session;

pub fn load_session(model_path: String) -> anyhow::Result<Session> {
    let session = Session::builder()?.commit_from_file(model_path)?;
    Ok(session)
}
