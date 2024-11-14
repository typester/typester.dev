use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    path::Path,
    sync::{Arc, OnceLock},
};

use chrono::{DateTime, Datelike, FixedOffset};
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Entry {
    pub eid: String,
    #[serde(default)]
    pub slug: String,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub tags: Vec<String>,
    pub image: Option<String>,
    pub content: String,
}

impl Entry {
    pub fn permalink(&self) -> String {
        format!("/{}/{}", self.date.format("%Y/%m/%d"), self.slug)
    }
}

#[derive(Debug)]
pub struct EntryLoader {
    prefix: String,
    entries: Vec<Arc<Entry>>,
    permalinks: HashMap<String, Arc<Entry>>,
    tags: HashMap<String, Vec<Arc<Entry>>>,
}

static BY_YEAR_CACHE: OnceLock<BTreeMap<i32, Vec<Arc<Entry>>>> = OnceLock::new();

impl EntryLoader {
    pub fn load(prefix: String, data_dir: String) -> anyhow::Result<Arc<Self>> {
        let entries = load_entries(&data_dir)?;
        Self::from_entries(prefix, entries)
    }

    pub fn from_entries(prefix: String, mut entries: Vec<Entry>) -> anyhow::Result<Arc<Self>> {
        entries.sort_by(|a, b| b.date.cmp(&a.date));
        let entries = entries
            .into_iter()
            .map(Arc::new)
            .collect::<Vec<Arc<Entry>>>();

        let mut permalinks: HashMap<String, Arc<Entry>> = HashMap::new();
        let mut tags: HashMap<String, Vec<Arc<Entry>>> = HashMap::new();
        for entry in entries.iter() {
            permalinks.insert(format!("{}{}", prefix, entry.permalink()), entry.clone());
            for tag in &entry.tags {
                let t = tags.entry(tag.clone()).or_insert(vec![]);
                t.push(entry.clone());
            }
        }

        tracing::trace!(%prefix, ?entries, "entry loaded");

        Ok(Arc::new(EntryLoader {
            prefix,
            entries,
            permalinks,
            tags,
        }))
    }

    pub fn get_entry_for_path(&self, path: &str) -> Option<Arc<Entry>> {
        tracing::trace!(%path, map = ?self.permalinks, "get_entry_for_path");
        self.permalinks.get(path).cloned()
    }

    pub fn get_entries(&self) -> Vec<Arc<Entry>> {
        self.entries.clone()
    }

    pub fn get_entries_by_tag(&self, tag: &str) -> Option<Vec<Arc<Entry>>> {
        let entries = self.tags.get(tag).map(|t| t.clone());
        entries
    }

    pub fn get_entries_by_year(&self) -> BTreeMap<i32, Vec<Arc<Entry>>> {
        tracing::trace!("get_entries_by_year");
        let map = BY_YEAR_CACHE.get_or_init(|| {
            let mut map: BTreeMap<i32, Vec<Arc<Entry>>> = BTreeMap::new();
            for entry in self.entries.iter() {
                map.entry(entry.date.year())
                    .or_insert_with(Vec::new)
                    .push(entry.clone())
            }
            map
        });
        map.clone()
    }
}

fn load_entries<P: AsRef<Path> + Debug>(dir: P) -> anyhow::Result<Vec<Entry>> {
    let mut data = vec![];
    let re_slug = Regex::new(r"^(?:\d+-\d+-\d+_)?(.*)$")?;

    tracing::debug!(?dir, "load_entries");

    let mut iter = std::fs::read_dir(&dir)?;
    while let Some(file) = iter.next() {
        let path = match file {
            Ok(file) => file.path(),
            Err(err) => {
                tracing::error!(%err, "read_dir error");
                continue;
            }
        };

        if path.is_dir() {
            let mut d = load_entries(path)?;
            data.append(&mut d);
        } else if let Some(ext) = path.extension() {
            if ext == "json" {
                // skip if the basename is not available
                let Some(slug) = path.file_stem() else {
                    continue;
                };
                let Some(slug) = slug.to_str() else { continue };

                let slug = match re_slug.captures(slug) {
                    Some(caps) => caps[1].to_string(),
                    None => slug.to_string(),
                };

                let content = std::fs::read(&path)?;
                let mut entry = serde_json::from_slice::<Entry>(&content)?;
                entry.slug = slug;
                data.push(entry);
            }
        }
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::Entry;

    #[test]
    fn test_deserialize() {
        let json = r#"{"title":"エディタのフォントを変えた","date":"2019-02-12T09:52:00+09:00","tags":["emacs","font","source-han-code-jp"],"eid":"db2d6fe8-6c10-4f38-8f07-ad77afd26ec5","image":null,"content":"<p>Emacsのフォントは長くRictyを使っていたのだが、なんとなく変えたくなり、検索してみたところ、 <a href=\"https://github.com/adobe-fonts/source-han-code-jp\">Source Han Code JP</a> というフォントが良さそうだったので、それにしてみた。</p>\n<p>英字部分の Source Code Pro はなかなか見易くて気にいったのだけど、\n英字と日本語の幅のレートがが 1:2 ではなく、 1:1.5 になっていて、org-modeのテーブルが崩れてしまうのが残念。</p>\n<p>Emacsの日本語フォントをうまく設定すれば 1:2 にすることはできそうだけど、調べるのがめんどうでやりたくない…。</p>\n"}"#;

        let entry = serde_json::from_str::<Entry>(json).unwrap();
        assert_eq!(entry.title, "エディタのフォントを変えた");

        println!("date: {}", entry.date.format("%Y-%m-%dT%H:%M:%S%z"))
    }

    #[test]
    fn test_re() {
        let re_slug = Regex::new(r"^(?:\d+-\d+-\d+_)?(.*)$").unwrap();

        let caps = re_slug.captures("2024-10-10_dameleon").unwrap();
        assert_eq!(caps[1].to_string(), "dameleon");
    }
}
