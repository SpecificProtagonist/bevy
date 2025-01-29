use bevy_ecs::prelude::*;

#[derive(Component, Debug)]
#[component(change_detection = true)]
struct Tracked;

#[derive(Component, Debug)]
#[component(change_detection = false)]
struct Untracked;

fn for_loops(mut query: Query<(&mut Tracked, &mut Untracked)>) {
    for (tracked, untracked) in query.iter_mut() {
        tracked.set_changed();
        tracked.is_changed();
        untracked.set_changed();
        untracked.is_changed();
        //~^ E0599
    }
}
