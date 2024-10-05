pub mod Sigma {

    struct SigmaLogSource {
        name: String,
    }

    enum LogSource {
        Service(String),
        Categoty(String),
        Product(String),
    }

    enum SigmaStatus {
        Stable,
        Test,
        // TODO complete
    }

    enum SigmaLevel {
        Critical,
        High,
        Medium,
        Low,
        Informational,
    }

    struct SigmaMetadata {
        name: String,
        id: String,
        status: SigmaStatus,
        description: String,
        licence: String,
        author: String,
        references: Vec<String>,
        tags: Vec<String>,
        level: SigmaLevel,
    }

    struct SigmaDetecton {
        // TODO
    }

    pub struct SigmaRule {
        meta: SigmaMetadata,
        sources: Vec<LogSource>,
        detection: SigmaDetecton,
    }

    impl SigmaRule {
        pub fn parse_rule(&self) -> SigmaRule {
            todo!()
        }

        pub fn parse_rule_from_file(&self, path: String) -> SigmaRule {
            todo!()
        }
    }
}
