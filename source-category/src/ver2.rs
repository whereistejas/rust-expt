use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter)]
enum Forms {
    Kafka,
    CockroachDb,
    Oracle,
    Db2,
    MsSql,
    MongoDbV2,
    MySql,
    Postgres,
    Bigquery,
    Snowflake,
    Redshift,
    File,
    S3,
    GoogleSheets,
}

impl Forms {
    fn source_id(&self) -> String {}
    fn name(&self) -> String {}
    fn icon(&self) -> String {}
    fn enabled(&self) -> bool {}
    fn form(&self) -> TopLevelForm {}
    fn to_slug(&self) -> String {}
    fn from_slug(&self) -> Self {}

    fn category(&self) -> Categories {
        match self {
            Self::Kafka => Categories::Database,
            Self::CockroachDb => Categories::Database,
            Self::Oracle => Categories::TraditionalDatabase,
            Self::Db2 => Categories::Database,
            Self::MsSql => Categories::TraditionalDatabase,
            Self::MongoDbV2 => Categories::Database,
            Self::MySql => Categories::TraditionalDatabase,
            Self::Postgres => Categories::TraditionalDatabase,
            Self::Bigquery => Categories::Database,
            Self::Snowflake => Categories::Database,
            Self::Redshift => Categories::Database,
            Self::File => Categories::Database,
            Self::S3 => Categories::Database,
            Self::GoogleSheets => Categories::Database,
        }
    }
}

#[derive(EnumIter, PartialEq)]
enum Categories {
    EditorialisedSource,
    Database,
    File,
    Service,
    TraditionalDatabase,
}

impl Categories {
    fn category_id(&self) -> String {}
    fn name(&self) -> String {}
    fn icon(&self) -> String {}
    fn to_slug(&self) -> String {}
    fn from_slug(&self) -> Self {}

    fn category(&self) -> Categories {
        match self {
            Categories::EditorialisedSource => Categories::EditorialisedSource,
            Categories::Database => Categories::EditorialisedSource,
            Categories::File => Categories::EditorialisedSource,
            Categories::Service => Categories::EditorialisedSource,
            Categories::TraditionalDatabase => Categories::Database,
        }
    }
    fn render(&self) -> SectionOrModal {
        match self {
            Categories::EditorialisedSource => SectionOrModal::Modal,
            Categories::Database => SectionOrModal::Section,
            Categories::File => SectionOrModal::Section,
            Categories::Service => SectionOrModal::Section,
            Categories::TraditionalDatabase => SectionOrModal::Modal,
        }
    }
    fn children(&self) -> Vec<Children> {
        let forms = Forms::iter()
            .filter(|form| form.category() == *self)
            .map(|form| Children::Forms(form));
        let categories = Categories::iter()
            .filter(|category| category.category() == *self)
            .map(|category| Children::Categories(category));

        forms.chain(other).collect()
    }
}

enum SectionOrModal {
    Section,
    Modal,
}

enum Children {
    Forms(Forms),
    Categories(Categories),
}
