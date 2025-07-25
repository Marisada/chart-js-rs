use {
    crate::{objects::helper_objects::*, traits::*},
    serde::{de, Deserialize, Serialize},
    serde_json::Value,
    std::{collections::HashMap, fmt::Debug},
};

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SinglePointDataset {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) backgroundColor: Vec<String>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) barPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) barThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) base: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderJoinStyle: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) borderRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderSkipped: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) categoryPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) clip: NumberString,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) data: Vec<NumberString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) datalabels: Option<DataLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) grouped: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) hoverBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) hoverBorderColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) hoverBorderRadius: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) hoverBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) indexAxis: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) inflateAmount: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) label: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) maxBarThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) minBarLength: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) order: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointBorderColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pointBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointHoverBackgroundColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pointHoverBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) pointHoverRadius: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) pointRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointStyle: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) skipNull: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) stack: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stepped: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) xAxisID: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) yAxisID: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct XYDataset {
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    pub(crate) backgroundColor: FnWithArgsOrT<2, String>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default,
        rename(serialize = "backgroundColor")
    )]
    pub(crate) backgroundColorArray: Vec<String>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) barPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) barThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) base: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderJoinStyle: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) borderRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderSkipped: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) category_label: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) categoryPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) clip: NumberString,
    #[serde(skip_serializing_if = "DatasetData::is_empty", default)]
    pub(crate) data: DatasetData,
    /// Use Default::default() if this isn't required
    pub(crate) datalabels: DataLabels,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) description: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) fill: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) grouped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) hidden: Option<bool>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) hitRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) hoverBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) hoverBorderColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) hoverBorderRadius: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) hoverBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) axis: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) inflateAmount: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) label: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) maxBarThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) minBarLength: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) order: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointBorderColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pointBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) pointHitRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointHoverBackgroundColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pointHoverBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) pointHoverRadius: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) pointRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointStyle: String,
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) rotation: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) segment: Option<Segment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) showLine: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) skipNull: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) spanGaps: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) stack: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stepped: Option<BoolString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) tension: NumberString,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) r#type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) xAxisID: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) yAxisID: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) z: NumberString,
}
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FloatingDataset {
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    pub(crate) backgroundColor: FnWithArgsOrT<2, String>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default,
        rename(serialize = "backgroundColor")
    )]
    pub(crate) backgroundColorArray: Vec<String>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) barPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) barThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) base: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderJoinStyle: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) borderRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderSkipped: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) category_label: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) categoryPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) clip: NumberString,
    #[serde(skip_serializing_if = "DatasetData::is_empty", default)]
    pub(crate) data: DatasetData,
    /// Use Default::default() if this isn't required
    pub(crate) datalabels: DataLabels,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) description: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) fill: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) grouped: Option<bool>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) hitRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) hoverBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) hoverBorderColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) hoverBorderRadius: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) hoverBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) axis: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) inflateAmount: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) label: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) maxBarThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) minBarLength: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) order: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointBorderColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pointBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) pointHitRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointHoverBackgroundColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pointHoverBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) pointHoverRadius: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) pointRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointStyle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) segment: Option<Segment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) skipNull: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) spanGaps: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) stack: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stepped: Option<BoolString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) tension: NumberString,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) r#type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) xAxisID: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) yAxisID: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) z: NumberString,
}
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct XYPoint {
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) x: NumberOrDateString,

    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) y: NumberString,

    #[serde(skip_serializing_if = "serde_json::Value::is_null", default)]
    pub(crate) description: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct ChartOptions {
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) aspectRatio: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) elements: Option<ChartElements>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) interaction: Option<ChartInteraction>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) indexAxis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) legend: Option<ChartLegend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) layout: Option<ChartLayout>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) locale: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) maintainAspectRatio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) plugins: Option<ChartPlugins>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) responsive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) scales: Option<HashMap<String, ChartScale>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) spanGaps: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Animation {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) duration: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct ChartPlugins {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) annotation: Option<Annotations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) autocolors: Option<AutoColors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) legend: Option<PluginLegend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tooltip: Option<TooltipPlugin>,
    // https://github.com/chartjs/chartjs-plugin-zoom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) zoom: Option<PluginZoom>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PluginLegend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) labels: Option<LegendLabel>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) position: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reverse: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct Annotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) annotations: Option<HashMap<String, Annotation>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AutoColors {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TooltipPlugin {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) backgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) bodyAlign: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) bodyColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) callbacks: Option<TooltipCallbacks>,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub(crate) filter: FnWithArgs<1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) displayColors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enabled: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) titleAlign: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) titleColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) titleMarginBottom: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) usePointStyle: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) padding: Option<Padding>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TooltipCallbacks {
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub(crate) label: FnWithArgs<1>,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub(crate) title: FnWithArgs<1>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartScale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) adapters: Option<ScaleAdapters>,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub(crate) afterBuildTicks: FnWithArgs<1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) alignToPixels: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) backgroundColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) barPercentage: NumberString,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub(crate) beforeFit: FnWithArgs<1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) beginAtZero: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) border: Option<ScaleBorder>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) bounds: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) categoryPercentage: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) display: Option<bool>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) grace: NumberOrDateString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) grid: Option<Grid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) grouped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) labels: Option<Vec<NumberOrDateString>>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) max: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) min: NumberOrDateString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) offset: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) position: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reverse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stacked: Option<bool>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) suggestedMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) suggestedMin: NumberOrDateString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ticks: Option<ScaleTicks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) time: Option<ScaleTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<Title>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    #[serde(rename = "type")]
    pub(crate) r#type: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) weight: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleBorder {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) color: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) dash: Vec<NumberString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) dashOffset: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) display: Option<bool>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) width: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) z: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) drawOnChartArea: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) offset: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default, skip_deserializing)]
    // the skip_deserializing needed because chartjs sets a default with a different type, FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub(crate) tickColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) z: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Callout {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) backgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) display: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) position: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) start: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelAnnotation {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) backgroundColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) borderRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) callout: Option<Callout>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) color: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) content: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) drawTime: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", skip_deserializing)]
    // the skip_deserializing needed because chartjs sets a default with a different type
    pub(crate) padding: Option<Padding>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) position: String,
    #[serde(default, rename = "type")]
    pub(crate) r#type: LabelAnnotationType,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) textAlign: String,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) xValue: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) xAdjust: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) yValue: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) yAdjust: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) xMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) xMin: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) yMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) yMin: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) yScaleID: NumberString,
}
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelAnnotationType;

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineAnnotation {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) drawTime: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) label: Option<LabelAnnotation>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) mode: String,
    #[serde(default, rename = "type")]
    pub(crate) r#type: LineAnnotationType,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) scaleID: String,
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) value: NumberString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) xMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) xMin: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) yMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub(crate) yMin: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) yScaleID: NumberString,
}
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineAnnotationType;

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoxAnnotation {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) backgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) drawTime: String,
    #[serde(default, rename = "type")]
    pub(crate) r#type: BoxAnnotationType,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) xMax: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) xMin: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) yMax: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) yMin: NumberString,
}
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoxAnnotationType;

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleTime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) displayFormats: Option<DisplayFormats>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) unit: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) tooltipFormat: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayFormats {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) day: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) hour: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) minute: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) month: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) quarter: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) week: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) year: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleTicks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) autoSkip: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) align: String,
    #[serde(
        skip_serializing_if = "FnWithArgs::is_empty",
        default,
        skip_deserializing // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    )]
    pub(crate) callback: FnWithArgs<3>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) color: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) count: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) includeBounds: Option<bool>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) maxTicksLimit: NumberString,
    #[serde(skip_serializing_if = "Option::is_none", skip_deserializing)]
    // the skip_deserializing needed because chartjs sets a default with a different type
    pub(crate) padding: Option<Padding>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) precision: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) stepSize: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Title {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) font: Option<Font>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartInteraction {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) axis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) intersect: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartLegend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) labels: Option<LegendLabel>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) position: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LegendLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) boxHeight: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) boxWidth: Option<u32>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) color: String,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub(crate) filter: FnWithArgs<2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) font: Option<Font>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) pointStyle: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) pointStyleWidth: NumberString,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub(crate) sort: FnWithArgs<3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) useBorderRadius: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) usePointStyle: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartElements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) bar: Option<BarElementConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) line: Option<LineElementConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) point: Option<PointElementConfiguration>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BarElementConfiguration {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) borderRadius: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fill: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) hoverBorderWidth: Option<NumberStringOrT<Border>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineElementConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) cubicInterpolationMode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fill: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointElementConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) hitRadius: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) hoverBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) hoverRadius: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) radius: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataLabels {
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    pub(crate) align: FnWithArgsOrT<1, String>,
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    pub(crate) anchor: FnWithArgsOrT<1, String>,
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    pub(crate) backgroundColor: FnWithArgsOrT<1, String>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) borderRadius: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) clamp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) clip: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) color: String,
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    pub(crate) display: FnWithArgsOrT<1, BoolString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) drawTime: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) font: Option<Font>,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub(crate) formatter: FnWithArgs<2>,
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub(crate) offset: FnWithArgsOrT<1, NumberString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) opacity: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) padding: Option<Padding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) rotation: Option<i16>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) z: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Border {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) bottom: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) left: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) right: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) top: NumberString,
}
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Padding {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) bottom: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) left: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) right: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) top: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Font {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub(crate) family: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) lineHeight: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) size: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) style: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub(crate) weight: NumberString,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Segment {
    #[serde(
        skip_serializing_if = "FnWithArgs::is_empty",
        default,
        skip_deserializing // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    )]
    pub(crate) borderColor: FnWithArgs<1>,
    #[serde(
        skip_serializing_if = "FnWithArgs::is_empty",
        default,
        skip_deserializing // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    )]
    pub(crate) borderDash: FnWithArgs<1>,
}

//
impl DatasetTrait for Vec<SinglePointDataset> {
    fn labels(self) -> Vec<NumberOrDateString> {
        let mut vec = self
            .into_iter()
            .map(|spd| spd.label.into())
            .collect::<Vec<_>>();

        vec.sort_by(crate::get_order_fn);
        vec.dedup();
        vec
    }
}
impl DatasetTrait for Vec<XYDataset> {
    fn labels(self) -> Vec<NumberOrDateString> {
        let mut vec = self
            .into_iter()
            .filter_map(|xyd| {
                let data = xyd.data.0.as_array()?;
                // gloo_console::console_dbg!(&data);
                let keys = data
                    .iter()
                    .filter_map(|xy| xy.as_object())
                    .filter_map(|obj| obj.get("x"))
                    .filter_map(|x| {
                        x.as_str()
                            .map(|s| s.to_string())
                            .or(x.as_number().map(|num| num.to_string()))
                    })
                    .map(|x| x.into())
                    .collect::<Vec<NumberOrDateString>>();
                Some(keys)
            })
            .flatten()
            .collect::<Vec<_>>();
        // gloo_console::console_dbg!(&vec);

        vec.sort_by(crate::get_order_fn);
        vec.dedup();
        vec
    }
}
impl DatasetTrait for Vec<FloatingDataset> {
    fn labels(self) -> Vec<NumberOrDateString> {
        let mut vec = self
            .into_iter()
            .map(|spd| spd.label.into())
            .collect::<Vec<_>>();

        vec.sort_by(crate::get_order_fn);
        vec.dedup();
        vec
    }
}
//
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Annotation {
    Box(BoxAnnotation),
    Line(LineAnnotation),
    Label(LabelAnnotation),
}

impl Default for Annotation {
    fn default() -> Self {
        Self::Line(Default::default())
    }
}
impl From<BoxAnnotation> for Annotation {
    fn from(value: BoxAnnotation) -> Self {
        Self::Box(value)
    }
}
impl From<LineAnnotation> for Annotation {
    fn from(value: LineAnnotation) -> Self {
        Self::Line(value)
    }
}
impl From<LabelAnnotation> for Annotation {
    fn from(value: LabelAnnotation) -> Self {
        Self::Label(value)
    }
}
//
impl From<(NumberOrDateString, NumberString, Option<Value>)> for XYPoint {
    fn from((x, y, d): (NumberOrDateString, NumberString, Option<Value>)) -> Self {
        XYPoint {
            x,
            y,
            description: d.unwrap_or_default(),
        }
    }
}
//
impl Ord for XYPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x)
    }
}
//
impl PartialOrd for XYPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
//
impl Serialize for BoxAnnotationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("box")
    }
}
//
impl Serialize for LineAnnotationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("line")
    }
}
//
impl Serialize for LabelAnnotationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("label")
    }
}
//
impl<'de> Deserialize<'de> for BoxAnnotationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.to_lowercase().as_str() {
            "box" => Ok(BoxAnnotationType),
            other => Err(de::Error::custom(format!(
                "`{other}` is not a valid BoxAnnotationType."
            ))),
        }
    }
}
//
impl<'de> Deserialize<'de> for LineAnnotationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.to_lowercase().as_str() {
            "line" => Ok(LineAnnotationType),
            other => Err(de::Error::custom(format!(
                "`{other}` is not a valid LineAnnotationType."
            ))),
        }
    }
}
//
impl<'de> Deserialize<'de> for LabelAnnotationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.to_lowercase().as_str() {
            "label" => Ok(LabelAnnotationType),
            other => Err(de::Error::custom(format!(
                "`{other}` is not a valid LabelAnnotationType."
            ))),
        }
    }
}
//

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleAdapters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) date: Option<ScaleAdaptersDate>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleAdaptersDate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outputCalendar: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct PluginZoom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pan: Option<ZoomPan>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) limits: Option<HashMap<String, ZoomLimit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) zoom: Option<ZoomZoom>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomLimit {
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) min: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) max: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) minRange: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomPan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) mode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) modifierKey: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) overScaleMode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) scaleMode: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) threshold: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomZoom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) mode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) overScaleMode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) scaleMode: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) drag: Option<ZoomDragOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pinch: Option<ZoomPinchOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) wheel: Option<ZoomWheelOptions>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomDragOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) backgroundColor: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) borderColor: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) borderWidth: NumberString,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) drawTime: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) modifierKey: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) threshold: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomWheelOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) modifierKey: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) speed: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomPinchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enabled: Option<bool>,
}
