use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub cal: String,
    #[serde(rename = "cal_unit")]
    pub cal_unit: String,
    pub diets: Vec<Diet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diet {
    pub desayuno: Desayuno,
    pub comida: Comida,
    pub cena: Cena,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Desayuno {
    pub alimentos: Vec<Alimento>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alimento {
    pub name: String,
    pub group: String,
    pub quantity: String,
    pub unit: String,
    pub gr: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comida {
    pub alimentos: Vec<Alimento2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alimento2 {
    pub name: String,
    pub group: String,
    pub quantity: String,
    pub unit: String,
    pub gr: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cena {
    pub alimentos: Vec<Alimento3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alimento3 {
    pub name: String,
    pub group: String,
    pub quantity: String,
    pub unit: String,
    pub gr: String,
}

