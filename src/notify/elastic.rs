use chrono::{DateTime, Utc, TimeZone};
use compact_str::CompactString;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
use Elasticsearch;

use crate::detections::{utils, message::AlertMessage, configs::{StoredStatic, Action}};

lazy_static! {}
pub struct AlertElastic {
    // HashSet of ISO8601 format Datetime and ruleid pair
    pub already_sent_set: Option<HashSet<(DateTime<Utc>, CompactString)>>,
    pub elastic_searcher: Elasticsearch,
}

impl AlertElastic {
    pub fn load_csv(path: &Path) -> Self {
        if !path.exists() {
            //ファイルがない場合は初回起動もしくは設定誤りの2通りがあるため警告文は表示しない
            println!("log file of sent alert is not found, so create new file. path: {}", path.display());
            AlertElastic {
                already_sent_set: None
            }
        } else {
            match utils::read_csv(path.as_os_str().to_str().unwrap_or_default()) {
                Ok(line) => {
                    return AlertElastic {
                        already_sent_set: Some(HashSet::from_iter(
                        line.iter().map(|x| {
                            let check = (Utc.parse_from_str(&x[0], "%Y-%m-%dT%H:%M:%S%.fZ"),x[1].clone().into());
                            if check.0.is_err() {
                                return None;
                            }
                            Some((check.0.unwrap(), check.1))
                        }).map(|y| y.unwrap()
                        )
                    ))};
                },
                Err(e) => {
                    // ファイルの読み込みエラーが発生した場合
                    AlertMessage::alert(&e);
                    return AlertElastic {
                        already_sent_set: None
                    };
                }
            }
        }
    }

    pub fn send_alert(stored_static: StoredStatic ) {
        // 10 is Action::AlertElastic
        if Action::to_usize(stored_static.config.action.as_ref()) == 10 {


        }
    }


}

#[cfg(test)]
mod tests {
    use compact_str::CompactString;
    use std::{net::IpAddr, path::Path, str::FromStr};

    #[test]
    fn test_no_specified_geo_ip_option() {
    }
}
