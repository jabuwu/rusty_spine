use itertools::Itertools;

pub enum Target {
    Default,
    Wasm,
}

impl Target {
    pub fn flags(&self) -> Vec<String> {
        match self {
            Target::Default => vec![],
            Target::Wasm => vec!["--target".to_owned(), "wasm32-unknown-unknown".to_owned()],
        }
    }
}

pub struct Features(pub &'static [&'static str]);

impl Features {
    pub fn combination_flags(&self) -> Vec<Vec<String>> {
        let mut feature_combinations: Vec<Vec<String>> =
            vec![vec!["--no-default-features".to_owned()]];
        for k in 1..=self.0.len() {
            feature_combinations.extend::<Vec<_>>(
                self.0
                    .iter()
                    .combinations(k)
                    .map(|features| {
                        vec![
                            "--no-default-features".to_owned(),
                            "--features".to_owned(),
                            features.iter().join(","),
                        ]
                    })
                    .collect::<Vec<_>>(),
            );
        }
        feature_combinations
    }
}
