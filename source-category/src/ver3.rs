use strum::{EnumIter, IntoEnumIterator};

struct TopLevelForm;

#[derive(EnumIter)]
enum Form {
    MongoDB,
    MySQL,
    PostgreSQL,
    MSSQL,
    CockroachDB,
}
impl Form {
    fn category(&self) -> Option<Category> {
        match self {
            Form::MongoDB => Some(Category::Section(Section::Database)),
            Form::MySQL => Some(Category::Modal(Modal::TraditionalDB)),
            Form::PostgreSQL => Some(Category::Modal(Modal::TraditionalDB)),
            Form::MSSQL => Some(Category::Modal(Modal::TraditionalDB)),
            Form::CockroachDB => Some(Category::Section(Section::Database)),
        }
    }

    fn icon(&self) -> &'static str {
        todo!()
    }
    fn title(&self) -> &'static str {
        todo!()
    }
    fn to_slug(&self) -> &'static str {
        todo!()
    }
    fn from_slug(slug: &str) -> Self {
        todo!()
    }
    fn form(&self) -> TopLevelForm {
        todo!()
    }
    fn enabled(&self) -> bool {
        todo!()
    }
}

enum Category {
    Section(Section),
    Modal(Modal),
}

#[derive(EnumIter, PartialEq, Eq)]
enum Section {
    Database,
    File,
    Service,
}
impl Section {
    fn children(&self) -> Vec<SectionChildren> {
        let modal = Modal::iter().filter_map(|modal| match modal.category().unwrap() {
            Category::Section(section) if section == *self => Some(SectionChildren::Modal(modal)),
            _ => None,
        });
        let form = Form::iter().filter_map(|form| match form.category().unwrap() {
            Category::Section(section) if section == *self => Some(SectionChildren::Form(form)),
            _ => None,
        });

        modal.chain(form).collect()
    }
    fn category(&self) -> Option<Category> {
        match self {
            Section::Database => Some(Category::Modal(Modal::Root)),
            Section::File => Some(Category::Modal(Modal::Root)),
            Section::Service => Some(Category::Modal(Modal::Root)),
        }
    }

    fn title(&self) -> &'static str {
        todo!()
    }
    fn to_slug(&self) -> &'static str {
        todo!()
    }
    fn from_slug(slug: &str) -> Self {
        todo!()
    }
}
enum SectionChildren {
    Modal(Modal),
    Form(Form),
}

#[derive(EnumIter, PartialEq, Eq)]
enum Modal {
    Root,
    TraditionalDB,
}
impl Modal {
    fn children(&self) -> Vec<ModalChildren> {
        let section = Section::iter().filter_map(|section| match section.category().unwrap() {
            Category::Modal(modal) if modal == *self => Some(ModalChildren::Section(section)),
            _ => None,
        });
        let form = Form::iter().filter_map(|form| match form.category().unwrap() {
            Category::Modal(modal) if modal == *self => Some(ModalChildren::Form(form)),
            _ => None,
        });

        section.chain(form).collect()
    }
    fn category(&self) -> Option<Category> {
        match self {
            Modal::Root => None,
            Modal::TraditionalDB => Some(Category::Section(Section::Database)),
        }
    }

    fn icon(&self) -> &'static str {
        todo!()
    }
    fn title(&self) -> &'static str {
        todo!()
    }
    fn to_slug(&self) -> &'static str {
        todo!()
    }
    fn from_slug(slug: &str) -> Self {
        todo!()
    }
}
enum ModalChildren {
    Section(Section),
    Form(Form),
}
