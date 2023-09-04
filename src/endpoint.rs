use serde_json::Value;

const NBA_BASE_URL: &str = "https://stats.nba.com/stats";

pub trait NBAStatsEndpoint {

    fn endpoint_params(&self) -> String;

    fn endpoint_name(&self) -> String;

    fn get_endpoint_url(&self) -> String {
        format!("{}/{}?{}", NBA_BASE_URL, self.endpoint_name(), self.endpoint_params())
    }

    fn fetch_nba_json(&self) -> Value {
        let endpoint_url = self.get_endpoint_url();
        let r = ureq::get(&endpoint_url)
            .set("Host","stats.nba.com")
            .set("User-Agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:72.0) Gecko/20100101 Firefox/72.0")
            .set("Accept","application/json, text/plain, */*")
            .set("Accept-Language","en-US,en;q=0.5")
            .set("Accept-Encoding","gzip, deflate, br")
            .set("Connection","keep-alive")
            .set("Referer","https://stats.nba.com/")
            .set("Pragma","no-cache")
            .set("Cache-Control","no-cache")
            .call().expect("Failed to fetch data from nba server");
        r.into_json().expect("Failed to fetch data from nba server")
    }
}