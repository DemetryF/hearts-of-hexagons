use {
    crate::{
        Players,
        map::Map,
        plugins::{
            tick::{PostTick, Tick},
            trigger_attack,
        },
    },
    bevy::prelude::*,
    bevy_replicon::prelude::*,
    shared::*,
    std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, HashSet},
    },
};

const PROV_DISTANCE: usize = 1;

pub struct DivisionMovementPlugin;

impl Plugin for DivisionMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Tick,
            (calculate_path.before(trigger_attack), process_moving).chain(),
        )
        .add_systems(PostTick, clear_movement_block)
        .add_observer(moving_order);
    }
}

#[derive(Component)]
pub struct MovingOrder {
    pub to: HexagonPos,
}

fn moving_order(
    event: On<FromClient<MovingOrderEvent>>,
    divisions: Query<&Division>,
    players: ResMut<Players>,
    mut commands: Commands,
) {
    let client = event.client_id.entity().unwrap();
    let country = players.0[&client];
    let division = event.entity;

    let division = divisions.get(division).unwrap();

    if division.country != country {
        return;
    }

    commands
        .entity(event.entity)
        .insert(MovingOrder { to: event.to });
}

fn calculate_path(
    division: Option<Single<(Entity, &Division, &MovingOrder), Changed<MovingOrder>>>,
    map: Res<Map>,
    mut commands: Commands,
) {
    let Some((id, division, order)) = division.map(|d| d.into_inner()) else {
        return;
    };

    #[derive(Clone, Copy, PartialEq, Eq)]
    struct QueueElement(i32, HexagonPos);

    impl PartialOrd for QueueElement {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for QueueElement {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut g_score = HashMap::new();
    let mut parent = HashMap::new();

    queue.push(Reverse(QueueElement(
        0 + division.pos.manhattan_dist(order.to),
        division.pos,
    )));

    g_score.insert(division.pos, 0);

    while let Some(Reverse(QueueElement(_, current))) = queue.pop() {
        if current == order.to {
            break;
        }

        visited.insert(current);

        for neighbour in current.neighbours() {
            if !map.provs.contains_key(&neighbour) {
                continue;
            }

            let tentative_score = g_score[&current] + 1;

            if g_score
                .get(&neighbour)
                .is_none_or(|&score| tentative_score < score)
            {
                g_score.insert(neighbour, tentative_score);
                parent.insert(neighbour, current);

                queue.push(Reverse(QueueElement(
                    tentative_score + neighbour.manhattan_dist(order.to),
                    neighbour,
                )));
            }
        }

        if let Some(mut current) = parent.get(&order.to).copied() {
            // reversed path
            let mut provs = vec![order.to];

            while current != division.pos {
                provs.push(current);
                current = parent[&current];
            }

            commands.entity(id).insert(Path { provs, progress: 0 });

            return;
        }
    }

    println!("couldnt find path");
}

pub fn process_moving(
    divisions: Query<(Entity, &mut Path, &mut Division, Option<&MovementBlock>)>,
    mut commands: Commands,
) {
    for (entity, mut path, mut division, movement_block) in divisions {
        println!("process moving");

        if movement_block.is_some() {
            continue;
        }

        path.progress += 1;

        if path.progress == PROV_DISTANCE {
            path.progress = 0;

            let from = division.pos;

            division.pos = path.provs.pop().unwrap();

            commands.entity(entity).trigger(|entity| DivisionMoved {
                entity,
                from,
                to: division.pos,
            });

            if path.provs.is_empty() {
                commands.entity(entity).remove::<(Path, MovingOrder)>();
            }
        }
    }
}

fn clear_movement_block(blocked: Query<Entity, With<MovementBlock>>, mut commands: Commands) {
    for id in blocked {
        println!("clear movement block");
        commands.entity(id).remove::<MovementBlock>();
    }
}
