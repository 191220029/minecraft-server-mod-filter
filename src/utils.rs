use reqwest::IntoUrl;

pub async fn get_body<T>(url: T) -> Result<String, reqwest::Error>
where
    T: IntoUrl,
{
    let resp = reqwest::get(url).await?;
    resp.text().await
}
