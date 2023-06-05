use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter)]
pub enum MeasurementType {
    GenerateJson,
    DeserializeJson,
    IterateIteratively,
    IterateRecursively,
    SerializeJson,
    Total,
    TotalIncludeContextSwitch,
}