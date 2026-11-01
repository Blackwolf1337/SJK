//! On-foot PM_CheckDuck clearance, codemp/game/bg_pmove.c:4382-4406, and the JA+ one.
use crate::pmove::{MovementCollision, MovementState};
use crate::pmove_japlus::JaPlusRules;

/// `MINS_Z`: a standing player's feet.
const MINS_Z: f32 = -24.0;

/// Whether a ducked or rolling player may stand up: [`can_stand`] on most servers, the
/// JA+ game module's own check on a JA+ server ([`ja_plus_can_stand`]).
pub(crate) fn may_stand(
    state: &mut MovementState,
    collision: &impl MovementCollision,
    minimums: [f32; 3],
    content_mask: u32,
    ja_plus: JaPlusRules,
) -> bool {
    if ja_plus.enabled {
        ja_plus_can_stand(
            state,
            collision,
            minimums,
            content_mask,
            ja_plus.old_collision,
        )
    } else {
        can_stand(state, collision, minimums, content_mask)
    }
}

/// The stand-up check of the JA+ 2.4 B7 game module (`PM_JPCanStand`, as VisionJK's port
/// of that module has it). With `dmflags` 256 it is the original game's: the standing box
/// must not be all in solid where the player is. Otherwise `PM_CheckFixMaxs`: the box from
/// the feet to the waist is swept up by the standing height; when the head would hit
/// something and there is that much room below (in the air, or in water), the player
/// stands up moved down by it, else he stays down.
fn ja_plus_can_stand(
    state: &mut MovementState,
    collision: &impl MovementCollision,
    minimums: [f32; 3],
    content_mask: u32,
    old_collision: bool,
) -> bool {
    let origin = state.origin;
    if old_collision {
        let standing = [15.0, 15.0, state.standing_height];
        return !collision
            .trace(origin, minimums, standing, origin, content_mask)
            .all_solid;
    }
    let low = [minimums[0], minimums[1], MINS_Z];
    let up = collision.trace(
        origin,
        low,
        [15.0, 15.0, 0.0],
        [origin[0], origin[1], origin[2] + state.standing_height],
        content_mask,
    );
    if up.all_solid || up.start_solid {
        return false;
    }
    if up.fraction >= 1.0 {
        return true;
    }
    let down_distance = state.standing_height * (1.0 - up.fraction);
    let down = collision.trace(
        origin,
        low,
        [15.0, 15.0, state.crouching_height],
        [origin[0], origin[1], origin[2] - down_distance],
        content_mask,
    );
    if down.all_solid || down.start_solid || down.fraction < 1.0 {
        return false;
    }
    state.origin[2] -= down_distance;
    true
}

/// Sweep nine small head probes from the crouched top to the standing top.
/// PM_CanStand rejects allsolid or fraction < 1, not startsolid by itself.
pub(crate) fn can_stand(
    state: &MovementState,
    collision: &impl MovementCollision,
    minimums: [f32; 3],
    content_mask: u32,
) -> bool {
    // The ordinary player's horizontal hull is -15..15; inset each probe by five.
    let mut x = minimums[0] + 5.0;
    while x <= 10.0 {
        let mut y = minimums[1] + 5.0;
        while y <= 10.0 {
            let start = [
                state.origin[0] + x,
                state.origin[1] + y,
                state.origin[2] + state.crouching_height,
            ];
            let end = [start[0], start[1], state.origin[2] + state.standing_height];
            let trace = collision.trace(
                start,
                [-5.0, -5.0, -2.5],
                [5.0, 5.0, 0.0],
                end,
                content_mask,
            );
            if trace.all_solid || trace.fraction < 1.0 {
                return false;
            }
            y += 10.0;
        }
        x += 10.0;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pmove::MovementTrace;

    /// A floor and a ceiling, infinitely wide: a box is solid where it crosses either.
    struct Slab {
        floor: f32,
        ceiling: f32,
    }
    impl MovementCollision for Slab {
        fn trace(
            &self,
            start: [f32; 3],
            mins: [f32; 3],
            maxs: [f32; 3],
            end: [f32; 3],
            _: u32,
        ) -> MovementTrace {
            let (bottom, top) = (start[2] + mins[2], start[2] + maxs[2]);
            if bottom < self.floor || top > self.ceiling {
                return MovementTrace {
                    fraction: 0.0,
                    start_solid: true,
                    all_solid: true,
                    ..MovementTrace::miss(start)
                };
            }
            let rise = end[2] - start[2];
            let fraction = if rise > 0.0 && top + rise > self.ceiling {
                (self.ceiling - top) / rise
            } else if rise < 0.0 && bottom + rise < self.floor {
                (bottom - self.floor) / -rise
            } else {
                1.0
            };
            MovementTrace {
                fraction,
                end_position: std::array::from_fn(|i| start[i] + (end[i] - start[i]) * fraction),
                ..MovementTrace::miss(end)
            }
        }
    }

    const MINS: [f32; 3] = [-15.0, -15.0, -24.0];

    fn ducked() -> MovementState {
        MovementState {
            origin: [0.0, 0.0, 100.0],
            standing_height: 40.0,
            crouching_height: 16.0,
            movement_flags: 1,
            ..MovementState::default()
        }
    }

    fn ja_plus(old_collision: bool) -> JaPlusRules {
        JaPlusRules {
            enabled: true,
            old_collision,
            ..JaPlusRules::default()
        }
    }

    #[test]
    fn ja_plus_stands_up_in_the_open() {
        let mut state = ducked();
        let open = Slab {
            floor: -1_000.0,
            ceiling: 1_000.0,
        };
        assert!(may_stand(&mut state, &open, MINS, 1, ja_plus(false)));
        assert_eq!(state.origin[2], 100.0);
    }

    #[test]
    fn ja_plus_stands_up_lower_under_a_ceiling_with_room_below() {
        // Head room for 30 of the 40 units above the origin, nothing below: the player
        // stands 10 lower, where stock stays crouched.
        let air = Slab {
            floor: -1_000.0,
            ceiling: 130.0,
        };
        let mut state = ducked();
        assert!(may_stand(&mut state, &air, MINS, 1, ja_plus(false)));
        assert_eq!(state.origin[2], 90.0);
        let mut stock = ducked();
        assert!(!may_stand(
            &mut stock,
            &air,
            MINS,
            1,
            JaPlusRules::default()
        ));
        assert_eq!(stock.origin[2], 100.0);
    }

    #[test]
    fn ja_plus_stays_down_without_room_below() {
        let tunnel = Slab {
            floor: 76.0,
            ceiling: 130.0,
        };
        let mut state = ducked();
        assert!(!may_stand(&mut state, &tunnel, MINS, 1, ja_plus(false)));
        assert_eq!(state.origin[2], 100.0);
    }

    #[test]
    fn ja_plus_old_collision_only_rejects_a_box_all_in_solid() {
        // dmflags 256: the original game's test of the standing box where the player is.
        // This world calls a box crossing the ceiling all solid, so he stays down; the
        // check never moves him.
        let air = Slab {
            floor: -1_000.0,
            ceiling: 130.0,
        };
        let mut state = ducked();
        assert!(!may_stand(&mut state, &air, MINS, 1, ja_plus(true)));
        let open = Slab {
            floor: -1_000.0,
            ceiling: 1_000.0,
        };
        assert!(may_stand(&mut state, &open, MINS, 1, ja_plus(true)));
        assert_eq!(state.origin[2], 100.0);
    }
}
