//! Recent tracks API: client, request builder, and response types.

mod builder;
mod extended;

pub use builder::RecentTracksRequestBuilder;

use crate::api::fetch_utils::ResourceContainer;
use crate::types::{RecentTrack, RecentTrackExtended, UserRecentTracks, UserRecentTracksExtended};

impl ResourceContainer for UserRecentTracks {
    type ItemType = RecentTrack;

    fn total(&self) -> u32 {
        self.recenttracks.attr.total
    }

    fn items(self) -> Vec<Self::ItemType> {
        self.recenttracks.track
    }

    /// The track that plays now has no date: Last.fm adds it on top of `limit`.
    fn is_extra(item: &Self::ItemType) -> bool {
        item.date.is_none()
    }
}

impl ResourceContainer for UserRecentTracksExtended {
    type ItemType = RecentTrackExtended;

    fn total(&self) -> u32 {
        self.recenttracks.attr.total
    }

    fn items(self) -> Vec<Self::ItemType> {
        self.recenttracks.track
    }

    /// The track that plays now has no date: Last.fm adds it on top of `limit`.
    fn is_extra(item: &Self::ItemType) -> bool {
        item.date.is_none()
    }
}
