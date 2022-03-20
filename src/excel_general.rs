use calamine::{Reader, Sheets};
use wasm_bindgen::JsValue;

/// sheets: excel对象
/// title_row: 标题在多少行
/// rows_excluded: 数据排除多少行
pub fn run(
    mut sheets: Sheets,
    excluded_keyword: String,
) -> Result<JsValue, JsValue> {
    let sheet_names = sheets.sheet_names().to_vec();
    let r: js_sys::Map = js_sys::Map::new();
    for name in &sheet_names {
        let r_arr = js_sys::Array::new();

        let sheet = match sheets.worksheet_range(name) {
            Some(Ok(v)) => v,
            e => Err(format!("sheet_name[{}]有误: {:?}", name, e))?,
        };

        let col_title = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "M", "L", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "x", "Y", "Z"];

        let mut row_count = 0_usize;
        for row in sheet.rows() {
            row_count += 1;
            let mut col_count = 0_usize;
            let m: js_sys::Map = js_sys::Map::new();
            let mut have_data = false;
            for i in 0..row.len() {
                let t = col_title.get(col_count).map(|v|v.to_string()).unwrap_or_else(|| format!("({})", i));
                col_count += 1;
                let td = format!("{}{}", t, row_count);
                let v: JsValue = match &row[i] {
                    calamine::DataType::Float(v) => JsValue::from(*v),
                    calamine::DataType::Int(v) => JsValue::from(*v as i32),
                    v => {
                        let v = v.to_string();
                        if excluded_keyword != "" && v.replace(' ', "") == excluded_keyword {
                            break;
                        }
                        if v.trim() == "" {
                            JsValue::NULL
                        } else {
                            v.into()
                        }
                    }
                };

                if v != JsValue::NULL {
                    have_data = true;
                }

                m.set(&JsValue::from_str(td.as_str()), &v);

            }

            if !have_data {
                break;
            }

            let m = js_sys::Object::from_entries(&m.into())?;
            r_arr.push(&m);
        }

        r.set(&JsValue::from_str(name.as_str()), &r_arr);
    }

    let r = js_sys::Object::from_entries(&r.into())?;
    Ok(r.into())
}
