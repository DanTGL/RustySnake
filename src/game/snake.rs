use bevy::prelude::*;

use super::*;
use super::Direction;
use super::Size;
use super::food::Food;
use super::food::FoodPlugin;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);


#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Deref, DerefMut, Resource)]
pub struct SnakeSegments(Vec<Entity>);

#[derive(Default, Resource)]
struct LastTailPosition(Option<Position>);

pub struct GrowthEvent;

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ]);
}

fn snake_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = {
            if keyboard_input.pressed(KeyCode::Left) {
                Direction::Left
            } else if keyboard_input.pressed(KeyCode::Down) {
                Direction::Down
            } else if keyboard_input.pressed(KeyCode::Up) {
                Direction::Up
            } else if keyboard_input.pressed(KeyCode::Right) {
                Direction::Right
            } else {
                head.direction
            }
        };

        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }

}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        }

        head_pos.x = head_pos.x.rem_euclid(ARENA_WIDTH as i32);
        head_pos.y = head_pos.y.rem_euclid(ARENA_HEIGHT as i32);

        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        
        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
    }

}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GrowthEvent>()
            .insert_resource(SnakeSegments::default())
            .insert_resource(LastTailPosition::default())
            .add_startup_system(spawn_snake)
            .add_system(snake_movement_input.before(snake_movement))
            .add_system(snake_movement.run_if(on_timer(Duration::from_secs_f32(0.150))))
            .add_system(snake_eating.after(snake_movement))
            .add_system(snake_growth.after(snake_eating))
            .add_systems((position_translation, size_scaling).in_base_set(CoreSet::PostUpdate))
            .add_plugin(FoodPlugin);
    
    }
}