use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use uuid::Uuid;

use crate::{app_state::{ViewState, DEFAULT_BIBLE}, audio::{reader_behavior::ReaderBehavior, TtsSettings}, bible::{Bible, ChapterIndex}, cloud_sync::sync_state::CloudSyncStateSave, migration::{self, MigrationResult}, notes::action::ActionHistory, settings::Settings};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppSaveVersion
{
    #[serde(rename = "0")]
    SV0,
}

impl AppSaveVersion
{
    pub const CURRENT_SAVE_VERSION: Self = AppSaveVersion::SV0;
}


#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct AppSave 
{
    pub note_record_saves: Vec<NotebookRecordSave>,
    pub local_device_save: LocalDeviceSave,
}

impl AppSave
{
    pub fn load(json: &str, bibles: &HashMap<String, impl AsRef<Bible>>) -> (AppSave, bool) 
    {
        let (migrated_json, was_migrated) = match migration::migrate_save_latest(json, bibles) {
            MigrationResult::Same(str) => (str, false),
            MigrationResult::Different {
                start,
                end,
                migrated,
            } => {
                println!("Migrated from {:?} to {:?}", start, end);
                (migrated, true)
            }
            MigrationResult::Error(err) => panic!("Error on save | {}", err),
        };

        (serde_json::from_str(&migrated_json).unwrap(), was_migrated)
    }
}

impl Default for AppSave
{
    fn default() -> Self 
    {   
        Self 
        {
            local_device_save: LocalDeviceSave::default(),
            note_record_saves: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalDeviceSaveVersion
{
    #[serde(rename = "0")]
    SV0,
}

impl LocalDeviceSaveVersion
{
    pub const CURRENT_SAVE_VERSION: Self = LocalDeviceSaveVersion::SV0;
}


#[derive(Serialize, Deserialize)]
pub struct LocalDeviceSave
{
    pub save_version: LocalDeviceSaveVersion,
    pub current_bible_version: String,
    pub view_state_index: usize,
    pub view_states: Vec<ViewState>,
    pub editing_note: Option<String>,
    pub settings: Settings,
    pub selected_reading: u32,
    pub tts_settings: TtsSettings,
    pub reader_behavior: ReaderBehavior,
    pub recent_highlights: Vec<Uuid>,
    pub cloud_sync_save: CloudSyncStateSave
}

impl Default for LocalDeviceSave
{
    fn default() -> Self 
    {
        let chapter = ChapterIndex { book: 0, number: 0 };
        let view_states = vec![ViewState::Chapter {
            chapter,
            verse_range: None,
            scroll: 0.0,
        }];
        
        Self {
            current_bible_version: DEFAULT_BIBLE.to_owned(),
            save_version: LocalDeviceSaveVersion::CURRENT_SAVE_VERSION,
            view_state_index: 0,
            view_states,
            editing_note: None,
            settings: Settings::default(),
            selected_reading: 0,
            tts_settings: TtsSettings::default(),
            reader_behavior: ReaderBehavior::default(),
            recent_highlights: vec![],
            cloud_sync_save: CloudSyncStateSave::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotebookRecordSaveVersion
{
    #[serde(rename = "0")]
    SV0,
}

impl NotebookRecordSaveVersion
{
    pub const CURRENT_SAVE_VERSION: Self = NotebookRecordSaveVersion::SV0;
}

// Synced data stored in the cloud
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotebookRecordSave
{
    pub history: ActionHistory,
    pub save_version: NotebookRecordSaveVersion,
    pub owner_id: Option<String>,
}

impl Default for NotebookRecordSave
{
    fn default() -> Self {
        Self 
        { 
            history: ActionHistory::new(), 
            save_version: NotebookRecordSaveVersion::CURRENT_SAVE_VERSION,
            owner_id: None,
        }
    }
}