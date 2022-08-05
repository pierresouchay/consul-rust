use consul_oxide::{Client, Config, Session, SessionEntry};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[tokio::test]
async fn test_session_create() {
    let (client, unique_test_identifier) = set_up().await;

    assert_eq!(
        get_number_of_session_entries_with_matching_name(&client, &unique_test_identifier).await,
        0
    );

    let entry =
        SessionEntry { name: Some(unique_test_identifier.to_string()), ..Default::default() };

    let created_session_entry = client.create_session(entry, None).await.unwrap();

    assert_eq!(
        get_number_of_session_entries_with_matching_name(&client, &unique_test_identifier).await,
        1
    );

    tear_down(&client, &created_session_entry.id.unwrap()).await;
}

#[tokio::test]
async fn test_session_destory() {
    let (client, unique_test_identifier) = set_up().await;

    let entry =
        SessionEntry { name: Some(unique_test_identifier.to_string()), ..Default::default() };

    let created_session_entry = client.create_session(entry, None).await.unwrap();

    assert_eq!(
        get_number_of_session_entries_with_matching_name(&client, &unique_test_identifier).await,
        1
    );

    let created_session_entry_id = created_session_entry.id.unwrap();

    client.destroy_session(&created_session_entry_id, None).await.unwrap();

    assert_eq!(
        get_number_of_session_entries_with_matching_name(&client, &unique_test_identifier).await,
        0
    );

    tear_down(&client, &created_session_entry_id).await;
}

#[tokio::test]
async fn test_session_info() {
    let (client, unique_test_identifier) = set_up().await;

    let entry =
        SessionEntry { name: Some(unique_test_identifier.to_string()), ..Default::default() };

    let created_session_entry = client.create_session(entry, None).await.unwrap();

    let created_session_entry_id = created_session_entry.id.unwrap();

    let session_entries = client.get_session_info(&created_session_entry_id, None).await.unwrap();

    assert_eq!(session_entries.len(), 1);

    let session_entry = session_entries.get(0);

    assert_eq!(*session_entry.as_ref().unwrap().name.as_ref().unwrap(), unique_test_identifier);

    tear_down(&client, &created_session_entry_id).await;
}

#[tokio::test]
async fn test_session_list() {
    let (client, unique_test_identifier) = set_up().await;

    let entry_names = vec![
        format!("{}-1", unique_test_identifier),
        format!("{}-2", unique_test_identifier),
        format!("{}-3", unique_test_identifier),
    ];

    let mut session_ids = Vec::<String>::new();

    for entry_name in &entry_names {
        let entry = SessionEntry { name: Some(entry_name.to_string()), ..Default::default() };

        let created_session_entry = client.create_session(entry, None).await.unwrap();

        session_ids.push(created_session_entry.id.unwrap());
    }

    let session_entries = client.list_sessions(None).await.unwrap();

    let filtered_session_entries = session_entries
        .iter()
        .filter(|s| s.name.as_ref().unwrap().contains(&unique_test_identifier))
        .collect::<Vec<&SessionEntry>>();

    assert_eq!(filtered_session_entries.len(), 3);

    let mut filtered_session_entry_names = filtered_session_entries
        .iter()
        .map(|s| s.name.as_ref().unwrap().to_string())
        .collect::<Vec<String>>();

    filtered_session_entry_names.sort();

    assert_eq!(filtered_session_entry_names, entry_names);

    for session_id in session_ids {
        tear_down(&client, &session_id).await;
    }
}

// TODO: test session renew - this is currently broken in CI as the consul test
// instace is run inside Docker, so hostname::get() does not match the hostname
// of the test instance. #[tokio::test]
// async fn session_node_test() {
//     let (client, unique_test_identifier) = set_up().await;

//     let entry =
//         SessionEntry { name: Some(unique_test_identifier.to_string()),
// ..Default::default() };

//     let created_session_entry = client.create_session(entry,
// None).await.unwrap();     let created_session_entry_id =
// created_session_entry.id.unwrap();     let system_hostname =
// hostname::get().unwrap().into_string().unwrap();     let session_entries =
// client.list_session_for_node(&system_hostname, None).await.unwrap();

//     assert_eq!(
//         session_entries
//             .iter()
//             .filter(|s| s.name.as_ref().unwrap() == &unique_test_identifier)
//             .count(),
//         1
//     );

//     tear_down(&client, &created_session_entry_id).await;
// }

#[tokio::test]
async fn test_renew_session() {
    let (client, unique_test_identifier) = set_up().await;

    let entry = SessionEntry { name: Some(unique_test_identifier), ..Default::default() };

    let created_session_entry = client.create_session(entry, None).await.unwrap();

    let created_session_entry_id = created_session_entry.id.unwrap();

    client.renew_session(&created_session_entry_id, None).await.unwrap();

    tear_down(&client, &created_session_entry_id).await;
}

async fn set_up() -> (Client, String) {
    let config = Config::default();
    let client = Client::new(config);

    let unique_test_identifier: String =
        thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();

    (client, unique_test_identifier)
}

async fn tear_down(client: &Client, session_id: &str) {
    client.destroy_session(session_id, None).await.unwrap();
}

async fn get_number_of_session_entries_with_matching_name(
    client: &Client,
    unique_test_identifier: &str,
) -> usize {
    let session_entries = client.list_sessions(None).await.unwrap();
    session_entries.iter().filter(|s| s.name.as_ref().unwrap() == unique_test_identifier).count()
}
