//! A mob whose player target left the level (#1278): an open world keeps the
//! mob bound to that departed player, so it sees no player for its tick, and
//! its own target goal lets go (`TargetGoal.canContinueToUse` fails for a
//! player no longer in the level) on the goal selector's cadence.
use super::*;
use crate::mobs::world::{test_player, with_test_world};
use std::sync::Arc;
use sunrise_core::geometry::Vec3;

#[test]
#[ignore = "daily"]
fn a_target_that_left_the_level_is_dropped_by_the_mobs_own_goal() {
    // Without prepared resources the world reports the skip and runs nothing.
    with_test_world(&[], |w, types| {
        let kind = Arc::new(types.types["minecraft:zombie"].clone());
        let mut m = Mob::new(1, kind, Vec3::new(0.5, 64.0, 0.5), 0.0);
        crate::mobs::ai::register(&mut m);
        m.random = sunrise_core::random::LegacyRandom::new(0);
        let mut fixture = crate::health::TestLevel::new(0);
        let mut level = fixture.level();
        let mut events = Vec::new();
        let mut here = w.clone();
        here.player = Some(test_player(Vec3::new(4.5, 64.0, 0.5)));
        for _ in 0..200 {
            crate::mobs::ai::tick(&mut m, &here, &mut level, &mut events);
            if m.ai.target == Some(Who::Player) {
                break;
            }
        }
        assert_eq!(
            m.ai.target,
            Some(Who::Player),
            "the zombie takes the player"
        );
        // The player leaves: the bound mob sees no player from now on.
        let gone = w.clone();
        let mut ticks = 0;
        while m.ai.target.is_some() && ticks < 2 {
            crate::mobs::ai::tick(&mut m, &gone, &mut level, &mut events);
            ticks += 1;
        }
        assert_eq!(
            m.ai.target, None,
            "dropped within the selector's two-tick cadence"
        );
    });
}
