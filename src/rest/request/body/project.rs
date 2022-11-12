use serde::Serialize;

use crate::rest::{
    models::{Color, Project},
    ViewStyle,
};

#[derive(Serialize, Debug)]
pub struct CreateProjectBody {
    name: String,
    parent_id: Option<String>,
    color: Option<Color>,
    is_favorite: Option<bool>,
    view_style: ViewStyle,
}

impl CreateProjectBody {
    pub fn new(name: &str) -> CreateProjectBody {
        CreateProjectBody {
            name: name.to_string(),
            parent_id: None,
            color: None,
            is_favorite: None,
            view_style: ViewStyle::default(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_parent_id(&mut self, parent_id: &str) {
        self.parent_id = Some(parent_id.to_string());
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn set_is_favorite(&mut self, is_favorite: bool) {
        self.is_favorite = Some(is_favorite);
    }

    pub fn set_view_style(&mut self, view_style: ViewStyle) {
        self.view_style = view_style;
    }

    pub fn with_parent_id(mut self, parent_id: &str) -> CreateProjectBody {
        self.set_parent_id(parent_id);
        self
    }

    pub fn with_color(mut self, color: Color) -> CreateProjectBody {
        self.set_color(color);
        self
    }

    pub fn with_is_favorite(mut self, is_favorite: bool) -> CreateProjectBody {
        self.set_is_favorite(is_favorite);
        self
    }

    pub fn with_view_style(mut self, view_style: ViewStyle) -> CreateProjectBody {
        self.set_view_style(view_style);
        self
    }
}

impl From<&Project> for CreateProjectBody {
    fn from(project: &Project) -> CreateProjectBody {
        CreateProjectBody {
            name: project.name.clone(),
            parent_id: project.parent_id.clone(),
            color: Some(project.color.clone()),
            is_favorite: Some(project.is_favorite.clone()),
            view_style: project.view_style.clone(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct UpdateProjectBody {
    name: Option<String>,
    color: Option<Color>,
    is_favorite: Option<bool>,
    view_style: Option<ViewStyle>,
}

impl UpdateProjectBody {
    pub fn new() -> UpdateProjectBody {
        UpdateProjectBody {
            name: None,
            color: None,
            is_favorite: None,
            view_style: None,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn set_is_favorite(&mut self, is_favorite: bool) {
        self.is_favorite = Some(is_favorite);
    }

    pub fn set_view_style(&mut self, view_style: ViewStyle) {
        self.view_style = Some(ViewStyle::from(view_style));
    }

    pub fn with_name(mut self, name: &str) -> UpdateProjectBody {
        self.set_name(name);
        self
    }

    pub fn with_color(mut self, color: Color) -> UpdateProjectBody {
        self.set_color(color);
        self
    }

    pub fn with_is_favorite(mut self, is_favorite: bool) -> UpdateProjectBody {
        self.set_is_favorite(is_favorite);
        self
    }

    pub fn with_view_style(mut self, view_style: ViewStyle) -> UpdateProjectBody {
        self.set_view_style(view_style);
        self
    }
}

impl From<&Project> for UpdateProjectBody {
    fn from(project: &Project) -> UpdateProjectBody {
        UpdateProjectBody {
            name: Some(project.name.clone()),
            color: Some(project.color.clone()),
            is_favorite: Some(project.is_favorite.clone()),
            view_style: Some(project.view_style.clone()),
        }
    }
}
