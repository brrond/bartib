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

    let client = reqwest::blocking::Client::new();

    filtered_activities.iter().for_each(|activity| {
        if activity.end.is_none() {
            println!("Ignoring activity {}, as it's still in progress", activity);
            return;
        }

        let issue_key = activity.project.clone();
        let comment = activity.description.clone();
        let started = activity.start.format("%Y-%m-%dT%H:%M:%S.000+0200");
        let time_spent_seconds = (activity.end.unwrap() - activity.start).num_seconds();

        let body = format!(
            "
            {{
                \"comment\": {{
                    \"content\": [
                        {{
                            \"content\": [
                                {{
                                    \"text\": \"{comment}\",
                                    \"type\": \"text\"
                                }}
                            ],
                            \"type\": \"paragraph\"
                        }}
                    ],
                    \"type\": \"doc\",
                    \"version\": 1
                }},
                \"started\": \"{started}\",
                \"timeSpentSeconds\": {time_spent_seconds}
            }}
            "
        );
        // println!("Body to send: {}", body);
        println!(
            "Trarnsmitting a worklog for {} {}s",
            issue_key, time_spent_seconds
        );

        let res = client
            .post(format!(
                "{jira_server}/rest/api/3/issue/{issue_key}/worklog"
            ))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(body)
            .basic_auth(jira_user, Option::Some(jira_token))
            .send();

        if res.is_err() || (res.is_ok() && !res.unwrap().status().is_success()) {
            panic!("Error during pushing to Jira");
        }
    });

    println!("All items were successfully uploaded to Jira.");

    Ok(())
}
