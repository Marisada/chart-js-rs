use {
    crate::{objects::helper_objects::*, traits::*},
    serde::{de, Deserialize, Serialize},
    serde_json::Value,
    std::{collections::HashMap, fmt::Debug},
};

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SinglePointDataset {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub backgroundColor: Vec<String>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub barPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub barThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub base: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderJoinStyle: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderSkipped: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub categoryPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub clip: NumberString,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub data: Vec<NumberString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datalabels: Option<DataLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub hoverBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub hoverBorderColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hoverBorderRadius: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoverBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub indexAxis: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub inflateAmount: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub label: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub maxBarThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub minBarLength: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub order: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointBorderColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointHoverBackgroundColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointHoverBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub pointHoverRadius: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub pointRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointStyle: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipNull: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub stack: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepped: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub xAxisID: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub yAxisID: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct XYDataset {
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    pub backgroundColor: FnWithArgsOrT<2, String>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default,
        rename(serialize = "backgroundColor")
    )]
    pub backgroundColorArray: Vec<String>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub barPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub barThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub base: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderJoinStyle: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderSkipped: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub category_label: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub categoryPercentage: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub clip: NumberString,
    #[serde(skip_serializing_if = "DatasetData::is_empty", default)]
    pub data: DatasetData,
    /// Use Default::default() if this isn't required
    pub datalabels: DataLabels,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub description: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub fill: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hitRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub hoverBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub hoverBorderColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hoverBorderRadius: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoverBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub axis: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub inflateAmount: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub label: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub maxBarThickness: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub minBarLength: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub order: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointBackgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointBorderColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub pointHitRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointHoverBackgroundColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointHoverBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub pointHoverRadius: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub pointRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointStyle: String,
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub rotation: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Segment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub showLine: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipNull: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spanGaps: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub stack: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stepped: Option<BoolString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub tension: NumberString,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub r#type: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub xAxisID: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub yAxisID: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub z: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct XYPoint {
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub x: NumberOrDateString,

    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub y: NumberString,

    #[serde(skip_serializing_if = "serde_json::Value::is_null", default)]
    pub description: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct ChartOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub aspectRatio: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<ChartElements>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction: Option<ChartInteraction>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub indexAxis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<ChartLegend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<ChartLayout>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub locale: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainAspectRatio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<ChartPlugins>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scales: Option<HashMap<String, ChartScale>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spanGaps: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Animation {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub duration: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct ChartPlugins {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Annotations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocolors: Option<AutoColors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<PluginLegend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipPlugin>,
    // https://github.com/chartjs/chartjs-plugin-zoom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<PluginZoom>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PluginLegend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<LegendLabel>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub position: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct Annotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, Annotation>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AutoColors {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TooltipPlugin {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub backgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub bodyAlign: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub bodyColor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<TooltipCallbacks>,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub filter: FnWithArgs<1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayColors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub titleAlign: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub titleColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub titleMarginBottom: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usePointStyle: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TooltipCallbacks {
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub label: FnWithArgs<1>,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub title: FnWithArgs<1>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartScale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapters: Option<ScaleAdapters>,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub afterBuildTicks: FnWithArgs<1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignToPixels: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub backgroundColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub barPercentage: NumberString,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub beforeFit: FnWithArgs<1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beginAtZero: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<ScaleBorder>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub bounds: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub categoryPercentage: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub grace: NumberOrDateString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<Grid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<NumberOrDateString>>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub max: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub min: NumberOrDateString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub position: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacked: Option<bool>,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub suggestedMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub suggestedMin: NumberOrDateString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks: Option<ScaleTicks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<ScaleTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub weight: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleBorder {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub color: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub dash: Vec<NumberString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub dashOffset: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub width: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub z: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawOnChartArea: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default, skip_deserializing)]
    // the skip_deserializing needed because chartjs sets a default with a different type, FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub tickColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub z: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Callout {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub backgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub position: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub start: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelAnnotation {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub backgroundColor: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderRadius: NumberString,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callout: Option<Callout>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub color: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub content: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub drawTime: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", skip_deserializing)]
    // the skip_deserializing needed because chartjs sets a default with a different type
    pub padding: Option<Padding>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub position: String,
    #[serde(default, rename = "type")]
    pub r#type: LabelAnnotationType,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub textAlign: String,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub xValue: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub xAdjust: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub yValue: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub yAdjust: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub xMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub xMin: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub yMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub yMin: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub yScaleID: NumberString,
}
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelAnnotationType;

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineAnnotation {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub drawTime: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<AnnotationLabel>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub mode: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub scaleID: String,
    #[serde(default, rename = "type")]
    pub r#type: LineAnnotationType,
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub value: NumberString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub xMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub xMin: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub yMax: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberOrDateString::is_empty", default)]
    pub yMin: NumberOrDateString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub yScaleID: NumberString,
}
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineAnnotationType;

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoxAnnotation {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub backgroundColor: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub borderColor: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub borderDash: Vec<NumberString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub drawTime: String,
    #[serde(default, rename = "type")]
    pub r#type: BoxAnnotationType,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub xMax: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub xMin: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub yMax: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub yMin: NumberString,
}
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoxAnnotationType;

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleAdapters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<ScaleAdaptersDate>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleAdaptersDate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputCalendar: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleTime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayFormats: Option<DisplayFormats>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tooltipFormat: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub unit: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayFormats {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub day: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub hour: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub minute: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub month: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub quarter: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub week: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub year: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleTicks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoSkip: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub align: String,
    #[serde(
        skip_serializing_if = "FnWithArgs::is_empty",
        default,
        skip_deserializing // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    )]
    pub callback: FnWithArgs<3>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub color: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub count: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeBounds: Option<bool>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub maxTicksLimit: NumberString,
    #[serde(skip_serializing_if = "Option::is_none", skip_deserializing)]
    // the skip_deserializing needed because chartjs sets a default with a different type
    pub padding: Option<Padding>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub precision: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub stepSize: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Title {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<Font>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartInteraction {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub axis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersect: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartLegend {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<LegendLabel>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub position: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LegendLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boxHeight: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boxWidth: Option<u32>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub color: String,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub filter: FnWithArgs<2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<Font>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub pointStyle: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub pointStyleWidth: NumberString,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub sort: FnWithArgs<3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useBorderRadius: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usePointStyle: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChartElements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<BarElementConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<LineElementConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<PointElementConfiguration>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BarElementConfiguration {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderRadius: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoverBorderWidth: Option<NumberStringOrT<Border>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineElementConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub cubicInterpolationMode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointElementConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hitRadius: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoverBorderWidth: Option<NumberStringOrT<Border>>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub hoverRadius: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub radius: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataLabels {
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    pub align: FnWithArgsOrT<1, String>,
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    pub anchor: FnWithArgsOrT<1, String>,
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    pub backgroundColor: FnWithArgsOrT<1, String>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub borderRadius: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clamp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub color: String,
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    pub display: FnWithArgsOrT<1, BoolString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub drawTime: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<Font>,
    #[serde(skip_serializing_if = "FnWithArgs::is_empty", skip_deserializing)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub formatter: FnWithArgs<2>,
    #[serde(skip_serializing_if = "FnWithArgsOrT::is_empty", default)]
    // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    pub offset: FnWithArgsOrT<1, NumberString>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub opacity: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Padding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<i16>,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub z: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Border {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub bottom: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub left: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub right: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub top: NumberString,
}
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Padding {
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub bottom: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub left: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub right: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub top: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Font {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub family: String,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub lineHeight: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub size: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub style: NumberString,
    #[serde(skip_serializing_if = "NumberString::is_empty", default)]
    pub weight: NumberString,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Segment {
    #[serde(
        skip_serializing_if = "FnWithArgs::is_empty",
        default,
        skip_deserializing // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    )]
    pub borderColor: FnWithArgs<1>,
    #[serde(
        skip_serializing_if = "FnWithArgs::is_empty",
        default,
        skip_deserializing // FnWithArgs can't deser right now, might be solved in the future with a fancy serde deserializer
    )]
    pub borderDash: FnWithArgs<1>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnnotationLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub backgroundColor: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub color: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub content: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub position: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct PluginZoom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan: Option<ZoomPan>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<HashMap<String, ZoomLimit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<ZoomZoom>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomLimit {
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub min: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub max: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub minRange: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomPan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub mode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub modifierKey: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub overScaleMode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub scaleMode: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub threshold: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomZoom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub mode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub overScaleMode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub scaleMode: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag: Option<ZoomDragOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinch: Option<ZoomPinchOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wheel: Option<ZoomWheelOptions>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomDragOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub backgroundColor: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub borderColor: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub borderWidth: NumberString,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub drawTime: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub modifierKey: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub threshold: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomWheelOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub modifierKey: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub speed: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomPinchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
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
