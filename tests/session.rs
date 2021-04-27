extern crate consul;
use consul::session::{Session, SessionEntry};
use consul::{Client, Config};

extern crate rand;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[test]
fn session_create_test() {
    let (client, unique_test_identifier) = set_up();

    assert_eq!(
        get_number_of_session_entries_with_matching_name(&client, &unique_test_identifier),
        0
    );

    let entry = SessionEntry {
        Name: Some(unique_test_identifier.to_string()),
        ..Default::default()
    };

    let (created_session_entry, _) = client.create(&entry, None).unwrap();

    assert_eq!(
        get_number_of_session_entries_with_matching_name(&client, &unique_test_identifier),
        1
    );

    tear_down(&client, &created_session_entry.ID.unwrap());
}

#[test]
fn session_destroy_test() {
    let (client, unique_test_identifier) = set_up();

    let entry = SessionEntry {
        Name: Some(unique_test_identifier.to_string()),
        ..Default::default()
    };

    let (created_session_entry, _) = client.create(&entry, None).unwrap();

    assert_eq!(
        get_number_of_session_entries_with_matching_name(&client, &unique_test_identifier),
        1
    );

    let created_session_entry_id = created_session_entry.ID.unwrap();

    client.destroy(&created_session_entry_id, None).unwrap();

    assert_eq!(
        get_number_of_session_entries_with_matching_name(&client, &unique_test_identifier),
        0
    );

    tear_down(&client, &created_session_entry_id);
}

#[test]
fn session_info_test() {
    let (client, unique_test_identifier) = set_up();

    let entry = SessionEntry {
        Name: Some(unique_test_identifier.to_string()),
        ..Default::default()
    };

    let (created_session_entry, _) = client.create(&entry, None).unwrap();

    let created_session_entry_id = created_session_entry.ID.unwrap();

    let (session_entries, _) = client.info(&created_session_entry_id, None).unwrap();

    assert_eq!(session_entries.len(), 1);

    let session_entry = session_entries.iter().next().unwrap();

    assert_eq!(
        session_entry.Name.as_ref().unwrap(),
        &unique_test_identifier
    );

    tear_down(&client, &created_session_entry_id);
}

#[test]
fn session_list_test() {
    let (client, unique_test_identifier) = set_up();

    let entry_names = vec![
        format!("{}-1", unique_test_identifier),
        format!("{}-2", unique_test_identifier),
        format!("{}-3", unique_test_identifier),
    ];

    let mut session_ids = Vec::<String>::new();

    for entry_name in &entry_names {
        let entry = SessionEntry {
            Name: Some(entry_name.to_string()),
            ..Default::default()
        };

        let (created_session_entry, _) = client.create(&entry, None).unwrap();

        session_ids.push(created_session_entry.ID.unwrap());
    }

    let (session_entries, _) = client.list(None).unwrap();

    let filtered_session_entries = session_entries
        .iter()
        .filter(|s| s.Name.as_ref().unwrap().contains(&unique_test_identifier))
        .collect::<Vec<&SessionEntry>>();

    assert_eq!(filtered_session_entries.len(), 3);

    let mut filtered_session_entry_names = filtered_session_entries
        .iter()
        .map(|s| s.Name.as_ref().unwrap().to_string())
        .collect::<Vec<String>>();

    filtered_session_entry_names.sort();

    assert_eq!(filtered_session_entry_names, entry_names);

    for session_id in session_ids {
        tear_down(&client, &session_id);
    }
}

#[test]
fn session_node_test() {
    let (client, unique_test_identifier) = set_up();

    let entry = SessionEntry {
        Name: Some(unique_test_identifier.to_string()),
        ..Default::default()
    };

    let (created_session_entry, _) = client.create(&entry, None).unwrap();

    let created_session_entry_id = created_session_entry.ID.unwrap();

    let system_hostname = hostname::get().unwrap().into_string().unwrap();

    let (session_entries, _) = client.node(&system_hostname, None).unwrap();

    let filtered_session_entries = session_entries
        .iter()
        .filter(|s| s.Name.as_ref().unwrap() == &unique_test_identifier)
        .collect::<Vec<&SessionEntry>>();

    assert_eq!(filtered_session_entries.len(), 1);

    tear_down(&client, &created_session_entry_id);
}

#[test]
fn session_renew_test() {
    let (client, unique_test_identifier) = set_up();

    let entry = SessionEntry {
        Name: Some(unique_test_identifier.to_string()),
        ..Default::default()
    };

    let (created_session_entry, _) = client.create(&entry, None).unwrap();

    let created_session_entry_id = created_session_entry.ID.unwrap();

    client.renew(&created_session_entry_id, None).unwrap();

    tear_down(&client, &created_session_entry_id);
}

fn set_up() -> (Client, String) {
    let config = Config::new().unwrap();
    let client = Client::new(config);

    let unique_test_identifier: String = thread_rng().sample_iter(&Alphanumeric).take(16).collect();

    return (client, unique_test_identifier);
}

fn tear_down(client: &Client, session_id: &str) {
    client.destroy(session_id, None).unwrap();
}

fn get_number_of_session_entries_with_matching_name(
    client: &Client,
    unique_test_identifier: &str,
) -> usize {
    let (session_entries, _) = client.list(None).unwrap();

    let filtered_session_entries = session_entries
        .iter()
        .filter(|s| s.Name.as_ref().unwrap() == &unique_test_identifier)
        .collect::<Vec<&SessionEntry>>();

    return filtered_session_entries.len();
}
