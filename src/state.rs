use tui::widgets::ListState;

#[derive(Debug)]
pub struct ProjectData{
    name: String,
    pods: Option<PodList>
}

impl Default for ProjectData {
    fn default() -> Self {
        Self { name: Default::default(), pods: None }
    }
}

#[derive(Debug)]
pub struct ProjectsList{
    state: ListState,
    items: Vec<ProjectData>
}

#[derive(Debug)]
pub struct PodList{
    state: ListState,
    pods: Vec<String>
}
