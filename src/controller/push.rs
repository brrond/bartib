use crate::data::{activity, bartib_file, getter, processor};
use anyhow::Result;

pub fn jira(
    file_name: &str,
    filter: getter::ActivityFilter,
    processors: processor::ProcessorList,
    jira_server: &str,
    jira_user: &str,
    jira_token: &str,
) -> Result<()> {
    let file_content = bartib_file::get_file_content(file_name)?;
    let activities = getter::get_activities(&file_content).collect();
    let processed_activities_bind: Vec<activity::Activity> =
        processor::process_activities(activities, processors);
    let processed_activities: Vec<&activity::Activity> = processed_activities_bind.iter().collect();

    let mut filtered_activities: Vec<&activity::Activity> =
        getter::filter_activities(processed_activities, &filter);
    filtered_activities.sort_by_key(|activity| activity.start);

    Ok(())
}
