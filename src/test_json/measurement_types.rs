#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MeasurementType {
    GenerateJson,
    DeserializeJson,
    IterateIteratively,
    IterateRecursively,
    SerializeJson,
    Total,
    TotalIncludeContextSwitch,
}