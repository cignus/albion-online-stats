use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

#[cfg(test)]
use fake_clock::FakeClock as Instant;
#[cfg(not(test))]
use std::time::Instant;

use cpython::PythonObject;
use cpython::ToPyObject;
use cpython::Python;
use cpython::PyObject;
use cpython::PyDict;
use cpython::PyList;

use super::traits::DamageStats;
use super::traits::FameStats;

#[derive(Debug, PartialEq, Clone)]
pub struct Party {
    pub id: usize,
    pub members: HashSet<String>
}

impl Party {
    pub fn new(id: usize, members: &std::vec::Vec<std::string::String>) -> Self {
        Self {
            id,
            members: HashSet::from_iter(members.iter().cloned()),
        }
    }

    pub fn add_member(&mut self, member_name: &str) {
        self.members.insert(member_name.to_string());
    }

    pub fn includes(&self, other: &str) -> bool {
        return self.members.contains(other);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerStatistics {
    pub player: String,
    pub damage: f32,
    pub time_in_combat: f32,
    pub dps: f32,
    pub seconds_in_game: f32,
    pub fame: f32,
    pub fame_per_minute: u32,
    pub fame_per_hour: u32
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerStatisticsVec {
    _vec: Vec<PlayerStatistics>
}

impl PlayerStatisticsVec {
    pub fn new() -> Self {
        Self {
            _vec: vec![]
        }
    }

    pub fn from(player_statistics_vec: Vec<PlayerStatistics>) -> Self {
        Self {
            _vec: player_statistics_vec
        }
    }

    pub fn merged(a: &Self, b: &Self) -> Self {
        let merged = [&a._vec[..], &b._vec[..]].concat().iter().fold(
            HashMap::<String, PlayerStatistics>::new(),
            |mut acc, stat| {
                acc.entry(stat.player.clone())
                    .and_modify(|s| {
                        s.damage += stat.damage;
                        s.time_in_combat += stat.time_in_combat;
                        s.dps = s.dps();
                        s.seconds_in_game += stat.seconds_in_game;
                        s.fame += stat.fame;
                        s.fame_per_minute = s.fame_per_minute();
                        s.fame_per_hour = s.fame_per_hour();
                    })
                    .or_insert(stat.clone());
                acc
            },
        );

        Self {
            _vec: merged.iter().map(|(_, v)| v.clone()).collect()
        }
    }
}

impl DamageStats for PlayerStatistics {
    fn damage(&self) -> f32 {
        self.damage
    }
    fn time_in_combat(&self) -> f32 {
        self.time_in_combat
    }
}

impl FameStats for PlayerStatistics {
    fn fame(&self) -> f32 {
        self.fame
    }

    fn time_started(&self) -> Instant {
        Instant::now()
    }

    fn time_in_game(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.seconds_in_game as u64)
    }
}

impl ToPyObject for PlayerStatistics {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        let stats = PyDict::new(py);

        stats
            .set_item(py, "player", self.player.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "damage", self.damage.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "time_in_combat", self.time_in_combat.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "dps", self.dps.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "seconds_in_game", self.seconds_in_game.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "fame", self.fame.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "fame_per_minute", self.fame_per_minute.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "fame_per_hour", self.fame_per_hour.to_py_object(py))
            .unwrap();

        stats.into_object()
    }
}

impl ToPyObject for PlayerStatisticsVec {
    type ObjectType = PyList;

    fn into_py_object(self, py: Python) -> Self::ObjectType {
        self._vec.into_py_object(py)
    }

    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        self._vec.clone().into_py_object(py)
    }
}