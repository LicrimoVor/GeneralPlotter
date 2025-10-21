use crate::libs::types::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SensorData {
    pub serial: Vec<String>,
    pub parsed: Vec<Value>,

    pub t_win: Vec<f64>,
    pub t_serial: Vec<f64>,

    pub all_serials: Vec<Vec<String>>,
    pub all_parseds: Vec<Vec<Value>>,
    pub all_points: Vec<Vec<[f64; 2]>>,
}

impl SensorData {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        if let Some(storage) = storage {
            eframe::get_value(storage, crate::core::consts::KEY_DATA).unwrap_or_default()
        } else {
            SensorData::default()
        }
    }

    pub fn add_data(
        &mut self,
        serial: Vec<String>,
        parsed: Vec<Value>,
        t_win: f64,
        t_serial: Option<f64>,
    ) {
        self.serial = serial.clone();
        self.parsed = parsed.clone();

        self.all_serials.push(serial.clone());
        self.all_parseds.push(parsed.clone());

        self.t_win.push(t_win);
        if let Some(t_serial) = t_serial {
            self.t_serial.push(t_serial);
        }

        let numbs = parsed
            .into_iter()
            .filter(|v| matches!(v, Value::Number(_)))
            .map(|v| match v {
                Value::Number(n) => n,
                _ => 0.0,
            })
            .collect::<Vec<f64>>();

        if numbs.len() != self.all_points.len() {
            self.all_points.push(vec![]);
        }

        for (i, numb) in numbs.iter().enumerate() {
            let points = self.all_points.get_mut(i).unwrap();
            points.push([t_win, *numb]);
        }
    }

    pub fn clear(&mut self) {
        self.all_serials.clear();
        self.all_parseds.clear();
        self.all_points.clear();

        self.t_win.clear();
        self.t_serial.clear();
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
