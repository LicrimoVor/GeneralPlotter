use crate::libs::types::{LinierFunc, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SensorData {
    pub serial: Vec<String>,
    pub parsed: Vec<Value>,

    pub t_win: Vec<f64>,
    pub t_serial: Vec<f64>,

    pub all_serials: Vec<Vec<String>>,
    pub all_originals: Vec<Vec<Value>>,
    pub all_parseds: Vec<Vec<Value>>,
    // t_win value, t_serial value
    pub all_points: Vec<(Vec<[f64; 2]>, Vec<[f64; 2]>)>,

    pub is_updated: bool,
    pub is_reload: bool,
}

impl SensorData {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        if let Some(storage) = storage {
            eframe::get_value(storage, crate::core::consts::KEY_DATA).unwrap_or_default()
        } else {
            SensorData::default()
        }
    }

    pub fn apply_linier(
        original: &Vec<Value>,
        linier_funcs: &Vec<Option<LinierFunc>>,
    ) -> Vec<Value> {
        original
            .iter()
            .enumerate()
            .map(|(k, v)| match v {
                Value::Number(numb) => {
                    if let Some(linier) = linier_funcs.get(k).unwrap() {
                        Value::Number(linier.value(*numb))
                    } else {
                        Value::Number(*numb)
                    }
                }
                _ => v.clone(),
            })
            .collect::<Vec<Value>>()
    }

    pub fn add_data(
        &mut self,
        serial: Vec<String>,
        original: Vec<Value>,
        parsed: Vec<Value>,
        t_win: f64,
        t_serial: Option<f64>,
    ) {
        self.serial = serial.clone();
        self.parsed = parsed.clone();

        self.all_serials.push(serial.clone());
        self.all_parseds.push(parsed.clone());
        self.all_originals.push(original.clone());

        self.t_win.push(t_win);
        if let Some(t_serial) = t_serial {
            self.t_serial.push(t_serial);
        } else {
            self.t_serial.push(0.0);
        }

        let numbs = parsed
            .into_iter()
            .filter(|v| matches!(v, Value::Number(_)))
            .map(|v| match v {
                Value::Number(n) => n,
                _ => 0.0,
            })
            .collect::<Vec<f64>>();

        if numbs.len() > self.all_points.len() {
            self.all_points.push((vec![], vec![]));
        }

        for (numb, points) in numbs.iter().zip(self.all_points.iter_mut()) {
            points.0.push([t_win, *numb]);
            if let Some(t_serial) = t_serial {
                points.1.push([t_serial, *numb]);
            } else {
                points.1.push([0.0, *numb]);
            }
        }

        self.is_updated = true;
    }

    pub fn reload(&mut self, linier_funcs: &Vec<Option<LinierFunc>>) {
        self.all_parseds.clear();
        self.all_points.clear();
        for _ in 0..self.all_originals[0].len() {
            self.all_points.push((vec![], vec![]));
        }

        for (original, (t_win, t_serial)) in self
            .all_originals
            .iter()
            .zip(self.t_win.iter().zip(self.t_serial.iter()))
        {
            let parsed = Self::apply_linier(original, linier_funcs);
            self.all_parseds.push(parsed.clone());
            let numbs = parsed
                .into_iter()
                .filter(|v| matches!(v, Value::Number(_)))
                .map(|v| match v {
                    Value::Number(n) => n,
                    _ => 0.0,
                })
                .collect::<Vec<f64>>();

            for (numb, points) in numbs.iter().zip(self.all_points.iter_mut()) {
                points.0.push([*t_win, *numb]);
                points.1.push([*t_serial, *numb]);
            }
        }
        self.is_reload = true;
    }

    pub fn clear(&mut self) {
        self.all_serials.clear();
        self.all_parseds.clear();
        self.all_points.clear();
        self.all_originals.clear();

        self.t_win.clear();
        self.t_serial.clear();
        self.is_reload = true;
    }

    // pub fn to_csv(&self) -> String {
    //     let mut csv = String::new();
    //     csv.push_str("time_win;time_serial;all_serials;all_parseds\n");

    //     let len = self
    //         .t_win
    //         .len()
    //         .min(self.t_serial.len())
    //         .min(self.all_serials.len())
    //         .min(self.all_parseds.len());

    //     for i in 0..len {
    //         let time_win = self.t_win[i];
    //         let time_serial = self.t_serial[i];

    //         let serials_str = serde_json::to_string(&self.all_serials[i]).unwrap_or_default();
    //         let parseds_str = serde_json::to_string(&self.all_parseds[i]).unwrap_or_default();

    //         csv.push_str(&format!(
    //             "{time_win};{time_serial};{serials_str};{parseds_str}\n"
    //         ));
    //     }

    //     csv
    // }
}
