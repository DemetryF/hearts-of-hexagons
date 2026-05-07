use {
    crate::{
        map::Map,
        plugins::{Tick, process_moving},
    },
    bevy::prelude::*,
    rand::seq::IteratorRandom,
    rand_distr::{Binomial, Distribution},
    shared::*,
};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Tick,
            (
                trigger_attack,
                apply_attacks,
                apply_defends,
                stop_attack,
                retreat,
                clear_attacks_defends,
            )
                .chain()
                .before(process_moving),
        );
    }
}

pub fn trigger_attack(
    moving_divisions: Query<(Entity, &Division, &Path)>,
    divisions: Query<(Entity, &Division)>,
    mut commands: Commands,
) {
    for (id, division, path) in moving_divisions {
        let &dst = path.provs.last().unwrap();

        let mut divs_at_dst = {
            (divisions.iter())
                .filter(|(_, div)| div.pos == dst)
                .peekable()
        };

        if divs_at_dst.peek().is_some() {
            commands.entity(id).insert(AttacksOn(dst));
            println!("trigger attack");
        }

        for (id, _) in divs_at_dst {
            commands.entity(id).insert(DefendsFrom(division.pos));
        }
    }
}

fn apply_attacks(
    attacking: Query<(&AttacksOn, &CombatStats)>,
    mut divisions: Query<(&mut Division, &CombatStats)>,
) {
    for (&AttacksOn(dst), &CombatStats { attack, .. }) in attacking {
        println!("apply attacks");

        let defenders = divisions.iter_mut().filter(|(d, _)| d.pos == dst);

        let (mut attacked, &CombatStats { defend, .. }) =
            defenders.choose(&mut rand::rng()).unwrap();

        deal_damage(&mut *attacked, attack, defend);
    }
}

fn apply_defends(
    defending: Query<(&DefendsFrom, &CombatStats)>,
    mut divisions: Query<(&mut Division, &CombatStats)>,
) {
    for (&DefendsFrom(dst), &CombatStats { attack, .. }) in defending {
        let attackers = divisions.iter_mut().filter(|(d, _)| d.pos == dst);

        let (mut attacked, &CombatStats { breakthrough, .. }) =
            attackers.choose(&mut rand::rng()).unwrap();

        deal_damage(&mut *attacked, attack, breakthrough);
    }
}

fn deal_damage(attacked: &mut Division, attack: f32, defend: f32) {
    let covered = f32::min(defend, attack);
    let uncovered = f32::max(attack - defend, 0.);

    let covered = Binomial::new(covered.floor() as u64, 0.1)
        .unwrap()
        .sample(&mut rand::rng());

    let uncovered = Binomial::new(uncovered.floor() as u64, 0.4)
        .unwrap()
        .sample(&mut rand::rng())
        + rand::random_bool(uncovered.fract() as f64) as u64;

    let hits = (covered + uncovered) as f32;

    attacked.hp -= hits * 0.1;
    attacked.organization -= hits * 0.15;

    attacked.hp = attacked.hp.max(0.);
    attacked.organization = attacked.organization.max(0.);
}

fn clear_attacks_defends(
    attacks: Query<Entity, With<AttacksOn>>,
    defends: Query<Entity, With<DefendsFrom>>,
    mut commands: Commands,
) {
    for id in attacks {
        commands.entity(id).remove::<AttacksOn>();
    }

    for id in defends {
        commands.entity(id).remove::<DefendsFrom>();
    }
}

fn stop_attack(attacking: Query<(Entity, &Division), With<AttacksOn>>, mut commands: Commands) {
    for (id, attacking) in attacking {
        if attacking.organization > 0. {
            continue;
        }

        commands.entity(id).remove::<AttacksOn>();
    }
}

fn retreat(
    defending: Query<(Entity, &Division), With<DefendsFrom>>,
    owners: Query<&Owner>,
    map: Res<Map>,
    mut commands: Commands,
) {
    for (id, defending) in defending {
        if defending.organization > 0. {
            continue;
        }

        println!("retreat");

        for neighbor in defending.pos.neighbours() {
            let Some(&prov) = map.provs.get(&neighbor) else {
                continue;
            };

            let owner = owners.get(prov).unwrap();

            if Some(defending.country) != owner.0 {
                continue;
            }

            commands
                .entity(id)
                .insert(MovingOrderComponent { to: neighbor });

            break;
        }
    }
}
