use clap::Parser;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    chart_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Chart {
    charts_list: ChartsList
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChartsList {
    data: Vec<App>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct App {
    attributes: AppAttributes
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppAttributes {
    name: String,
    platform_attributes: Option<PlatformAttributes>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlatformAttributes {
    ios: iOSAttributes,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case, non_camel_case_types)]
struct iOSAttributes {
    isXROSCompatible: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let chart_url = args.chart_url;

    let response = reqwest::get(&chart_url).await?.text().await?;

    let dom = Html::parse_document(&response);
    let selector = Selector::parse("script[type=\"fastboot/shoebox\"]").unwrap();
    let element = dom.select(&selector).last().unwrap();
    let chart: Chart = serde_json::from_str(element.inner_html().as_str()).unwrap();

    let mut i = 0;
    for app in chart.charts_list.data {
        let status_emoji = if let Some(platform_attributes) = app.attributes.platform_attributes {
            if platform_attributes.ios.isXROSCompatible {
                "✅"
            } else {
                "❌"
            }
        } else {
            "🤔"
        };

        println!("{}: ({}) {}", status_emoji, i + 1, app.attributes.name);
        i += 1;
    }

    Ok(())
}