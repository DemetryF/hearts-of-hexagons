use {
    crate::{
        components::{Division, HoveredProvince, Map, SelectedDivision},
        hexagon_pos::HexagonPos,
    },
    bevy::prelude::*,
    std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap, HashSet},
    },
};

const PROV_DISTANCE: usize = 1;

#[derive(Component)]
pub struct MovingOrder {
    pub to: HexagonPos,
}

#[derive(Component)]
pub struct Path {
    pub provs: Vec<HexagonPos>,
    pub progress: usize,
}

#[derive(EntityEvent)]
pub struct DivisionMoved {
    pub entity: Entity,

    pub from: HexagonPos,
    pub to: HexagonPos,
}

pub fn start_moving(
    selected: Option<Single<Entity, With<SelectedDivision>>>,
    hovered_province: Res<HoveredProvince>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    if input.just_pressed(MouseButton::Left)
        && let Some(hovered) = hovered_province.0
        && let Some(selected) = selected
    {
        commands
            .entity(*selected)
            .insert(MovingOrder { to: hovered });

        commands.entity(*selected).remove::<SelectedDivision>();
    }
}

pub fn calculate_path(
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
            if !map.provinces.contains_key(&neighbour) {
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
    divisions: Query<(Entity, &mut Path, &mut Division)>,
    mut commands: Commands,
) {
    for (entity, mut path, mut division) in divisions {
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
        }
    }
}

pub fn end_moving(divisions: Query<(Entity, &Path)>, mut commands: Commands) {
    for (id, path) in divisions {
        if path.provs.is_empty() {
            commands.entity(id).remove::<(Path, MovingOrder)>();
        }
    }
}
