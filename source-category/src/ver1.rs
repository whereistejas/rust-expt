#![allow(dead_code)]

use strum::{EnumIter, IntoEnumIterator};

mod traits {
    pub(crate) struct TopLevelForm;

    pub(crate) trait Form {
        fn title(&self) -> &'static str;
        fn icon(&self) -> &'static str;
        fn to_slug(&self) -> String;
        fn from_slug(source: &str) -> Result<Self, String>
        where
            Self: Sized;
        fn form(&self) -> TopLevelForm;
        fn enabled(&self) -> bool;
    }
}

#[derive(EnumIter)]
enum Forms {
    MongoDb,
    CockroachDB,
    MySQL,
    PostgreSQL,
}
#[derive(EnumIter)]
enum Sources {
    MongoDb,
    CockroachDB,
    MySQL,
    PostgreSQL,
    Database,
    TraditionalDatabase,
    Service,
    File,
}

struct Source {
    icon: String,
    name: String,
    category: SectionOrModal,
}
impl Sources {
    fn icon(&self) -> String {
        todo!()
    }
    fn name(&self) -> String {
        todo!()
    }

    fn category(&self) -> SectionOrModal {
        match self {
            Sources::MongoDb => SectionOrModal::Sections(Sections::Database),
            Sources::CockroachDB => SectionOrModal::Sections(Sections::Database),
            Sources::MySQL => SectionOrModal::Modals(Modals::TraditionalDatabase),
            Sources::PostgreSQL => SectionOrModal::Modals(Modals::TraditionalDatabase),
            Sources::Service => SectionOrModal::Modals(Modals::EditorialisedSources),
            Sources::File => SectionOrModal::Modals(Modals::EditorialisedSources),
            Sources::Database => SectionOrModal::Modals(Modals::EditorialisedSources),
            Sources::TraditionalDatabase => SectionOrModal::Sections(Sections::Database),
        }
    }
}

enum SectionOrModal {
    Sections(Sections),
    Modals(Modals),
}

#[derive(PartialEq, Eq)]
enum Sections {
    Database,
    File,
    Service,
}

struct Section {
    icon: String,
    name: String,
    sources: Vec<Sources>,
}
impl Sections {
    fn icon(&self) -> String {
        todo!()
    }
    fn name(&self) -> String {
        todo!()
    }
    fn sources(&self) -> Vec<Sources> {
        match self {
            Sections::Database => Sources::iter()
                .filter_map(|source| match source.category() {
                    SectionOrModal::Sections(section) => section.eq(self).then(|| source),
                    SectionOrModal::Modals(_) => None,
                })
                .collect::<Vec<_>>(),
            Sections::File => todo!(),
            Sections::Service => todo!(),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Modals {
    EditorialisedSources,
    TraditionalDatabase,
}

struct Modal {
    icon: String,
    name: String,
    description: String,
    sources: Vec<Sources>,
}
impl Modals {
    fn icon(&self) -> String {
        todo!()
    }
    fn name(&self) -> String {
        todo!()
    }
    fn description(&self) -> String {
        todo!()
    }
    fn sources(&self) -> Vec<Sources> {
        match self {
            Modals::TraditionalDatabase => Sources::iter()
                .filter_map(|source| match source.category() {
                    SectionOrModal::Sections(_) => None,
                    SectionOrModal::Modals(modal) => modal.eq(self).then(|| source),
                })
                .collect::<Vec<_>>(),
            Modals::EditorialisedSources => todo!(),
        }
    }
}
