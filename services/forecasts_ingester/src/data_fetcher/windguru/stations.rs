use super::WINDGURU_REFERER;
use crate::{
    actors::messages::ingesting::IngestMsg,
    data_fetcher::{client::FetchingClient, errors::FetchError},
    types::windguru::station::WindguruStationFetchParams,
};

async fn get_station_data(
    fetcher: &FetchingClient,
    params: WindguruStationFetchParams<'_>,
) -> Result<IngestMsg, FetchError> {
    let url = format!("{}/int/iapi.php", fetcher.url);

    let response = fetcher
        .client
        .get(url)
        .header("Referer", WINDGURU_REFERER)
        .query(&params)
        .send()
        .await?;

    let response_status = response.status().as_u16();
    tracing::debug!(response_status = response_status, "fetching station data");

    let station_data = IngestMsg::WindguruStationReading(response.json().await?);
    Ok(station_data)
}
