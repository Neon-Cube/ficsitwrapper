use std::{env, error::Error, fs};

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Getting Config Path...");
    let path =
        env::var("LOCALAPPDATA")? + r#"\FactoryGame\Saved\Config\Windows\GameUserSettings.ini"#;

    println!("Reading Config...");
    let config = fs::read_to_string(&path)?;

    println!("Overwriting Config...");
    fs::write(path, fix_screen_percentage(config))?;

    print!("Launching Game...");
    
    Ok(())
}

fn fix_screen_percentage(config: String) -> String {
    if let Some(screen_percentage) = get_float_value(&config) {
        config.replace(
            &format!(r#""r.ScreenPercentage", {screen_percentage}"#),
            r#""r.ScreenPercentage", 49.000000"#,
        )
    } else {
        let separator = if config.contains("mFloatValues=()") {
            ""
        } else {
            ","
        };

        config.replace(
            "mFloatValues=(",
            &format!(r#"mFloatValues=(("r.ScreenPercentage", 49.000000){separator}"#),
        )
    }
}

fn get_float_value(data: &str) -> Option<&str> {
    let key = r#""r.ScreenPercentage", "#;
    let start_index = data.find(key)? + key.len();
    Some(&data[start_index..start_index + 9])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_value() {
        assert_eq!(
            Some("75.000000"),
            get_float_value(r#"mFloatValues=(("r.ScreenPercentage", 75.000000))"#)
        );
        assert_eq!(None, get_float_value(r#"mFloatValues=()"#))
    }

    #[test]
    fn screen_percentage() {
        assert_eq!(
            r#"\
mIntValues=(("r.FullScreenMode", 0))
mFloatValues=(("r.ScreenPercentage", 49.000000))
mPrimaryLanguage="#
                .to_owned(),
            fix_screen_percentage(
                r#"\
mIntValues=(("r.FullScreenMode", 0))
mFloatValues=(("r.ScreenPercentage", 75.000000))
mPrimaryLanguage="#
                    .to_owned()
            )
        )
    }

    #[test]
    fn screen_percentage_empty_floatvalues() {
        assert_eq!(
            r#"\
mIntValues=(("r.FullScreenMode", 0))
mFloatValues=(("r.ScreenPercentage", 49.000000))
mPrimaryLanguage="#
                .to_owned(),
            fix_screen_percentage(
                r#"\
mIntValues=(("r.FullScreenMode", 0))
mFloatValues=()
mPrimaryLanguage="#
                    .to_owned()
            )
        )
    }

    #[test]
    fn screen_percentage_populated_floatvalues() {
        assert_eq!(
            r#"\
mIntValues=(("r.FullScreenMode", 0))
mFloatValues=(("r.ScreenPercentage", 49.000000),("FG.MouseSensitivity", 0.012056))
mPrimaryLanguage="#
                .to_owned(),
            fix_screen_percentage(
                r#"\
mIntValues=(("r.FullScreenMode", 0))
mFloatValues=(("FG.MouseSensitivity", 0.012056))
mPrimaryLanguage="#
                    .to_owned()
            )
        )
    }
}
