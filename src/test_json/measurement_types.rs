/* #region Imports */
// 3rd-Party
use strum_macros::EnumIter;
/* #endregion */

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
