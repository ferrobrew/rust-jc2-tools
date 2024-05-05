use bevy::prelude::*;
use bevy_jc2_file_system::FileSystemEvent;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct StreamingPlugin;

impl Plugin for StreamingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StreamingData>()
            .init_resource::<StreamingData>()
            .init_state::<StreamingState>()
            .add_systems(Last, update_streaming_state);
    }
}

#[derive(Resource, Debug, Clone, Default, Reflect)]
#[reflect(Resource)]
pub struct StreamingData {
    pending: Vec<PathBuf>,
    ready: Vec<PathBuf>,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StreamingState {
    #[default]
    Ready,
    Waiting,
}

fn update_streaming_state(
    mut data: ResMut<StreamingData>,
    mut events: EventReader<FileSystemEvent>,
    mut next_state: ResMut<NextState<StreamingState>>,
) {
    for event in events.read() {
        match event {
            FileSystemEvent::ArchivePending { path } => data.pending.push(path.clone()),
            FileSystemEvent::ArchiveUnmounted { path } => data.ready.retain(|f| f != path),
            FileSystemEvent::ArchiveMounted { path } => data.ready.retain(|f| f != path),
            FileSystemEvent::ArchiveError { path } => data.pending.retain(|f| f != path),
            _ => {}
        }
    }

    if data.pending.is_empty() {
        next_state.set(StreamingState::Ready);
    } else {
        next_state.set(StreamingState::Waiting);
    }
}
