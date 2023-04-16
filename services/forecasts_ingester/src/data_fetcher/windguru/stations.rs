use super::WINDGURU_REFERER;
use crate::{
    actors::messages::ingesting::IngestMsg,
    data_fetcher::{client::FetchingClient, errors::FetchError},
    types::windguru::station::WindguruStationFetchParams,
};

pub async fn get_station_data(
    fetcher: &FetchingClient,
    params: WindguruStationFetchParams,
) -> Result<IngestMsg, FetchError> {
    let url = format!("{}/int/iapi.php", fetcher.url);

    let request = fetcher
        .client
        .get(url)
        .header("Referer", WINDGURU_REFERER)
        .query(&params);

    tracing::debug!("station data request={:?}", request);

    let response = request.send().await?;

    let response_status = response.status().as_u16();
    tracing::debug!(response_status = response_status, "fetching station data");
    response.error_for_status_ref()?;

    let station_data = IngestMsg::WindguruStationReading(response.json().await?);
    Ok(station_data)
}
