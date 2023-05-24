use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::InputManagerBundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(InputManagerPlugin::<PlayerActions>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .add_system(update_system)
        .add_system(read_result_system)
        .run();
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerActions {
    Move,
    Jump,
}
#[derive(Clone, Default, Debug, Component)]
struct Player(String);

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    input: InputManagerBundle<PlayerActions>,
    rigid_body: RigidBody,
    controller: KinematicCharacterController,
    collider: Collider,
    spatial: SpatialBundle,
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands, win: Query<&Window>) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));

    let window = win.single();

    commands.spawn(PlayerBundle{
        player: Player {0: "Player".to_string()},
        input: InputManagerBundle::<PlayerActions> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), PlayerActions::Move)
                .insert(VirtualDPad::wasd(), PlayerActions::Move)
                .insert(KeyCode::Space, PlayerActions::Jump)
                .insert(GamepadButtonType::South, PlayerActions::Jump)
                .set_gamepad(Gamepad { id: 0 })
                .build(),
        },
        controller: KinematicCharacterController::default(),
        collider: Collider::cuboid(12., 20.),
        rigid_body: RigidBody::KinematicPositionBased,
        spatial: SpatialBundle {
                transform: Transform::from_xyz(10., 200., 0.0),
                ..default()
            }
    });
    // .insert(SpatialBundle {
    //     // transform: Transform::from_xyz(window.width() / 3.0, window.height(), 50.0),
    //     ..default()}
    // );
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn update_system(
    mut query: Query<(
        &ActionState<PlayerActions>,
        &mut KinematicCharacterController,
    ), With<Player>>,
) {
    for (action_state, mut controller) in query.iter_mut() {
        let mut y = -2.;
        let axis_vector = action_state
            .clamped_axis_pair(PlayerActions::Move)
            .unwrap().x();
        for action in action_state.get_just_pressed() {
            match action {
                PlayerActions::Jump => {
                    y = 150.
                },
                _ => (),
            }
        }
        
        controller.translation = Some(Vec2::new(axis_vector * 15., y));
    }
}

fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in controllers.iter() {
        println!(
            "Entity {:?} moved by {:?} and touches the ground: {:?}",
            entity, output.effective_translation, output.grounded
        );
    }
}




// I apologize for the confusion. Based on the updated information about the KinematicCharacterController in bevy_rapier2d version 0.21.0, you can indeed achieve player movement and jumping by setting the translation field of the KinematicCharacterController component.

// Here's an example code to demonstrate how to use the KinematicCharacterController for player movement and jumping:

// rust
// Copy code
// use bevy::prelude::*;
// use bevy_rapier2d::physics::KinematicCharacterController;

// struct Player;

// fn main() {
//     App::build()
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(setup.system())
//         .add_system(movement_system.system())
//         .run();
// }

// fn setup(commands: &mut Commands) {
//     commands
//         .spawn_bundle(SpriteBundle {
//             // ...
//         })
//         .insert(KinematicCharacterController::default())
//         .insert(Player);
// }

// fn movement_system(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(&Player, &mut KinematicCharacterController)>,
// ) {
//     for (_, mut controller) in query.iter_mut() {
//         let mut move_direction = Vec2::ZERO;

//         if keyboard_input.pressed(KeyCode::A) {
//             move_direction.x -= 1.0;
//         }
//         if keyboard_input.pressed(KeyCode::D) {
//             move_direction.x += 1.0;
//         }

//         let movement_speed = 5.0; // Movement speed

//         // Set the translation for the character
//         controller.translation = move_direction * movement_speed;

//         let jump_force = 10.0; // Jump force

//         if keyboard_input.just_pressed(KeyCode::Space) {
//             // Trigger jump
//             controller.translation.y += jump_force;
//         }
//     }
// }
// In this example, we add the KinematicCharacterController component to the player entity and modify its translation field based on player input. The translation field represents the desired movement for the character. Additionally, we handle the jump by modifying the translation.y value when the spacebar is pressed.

// During the physics update step, the KinematicCharacterController will resolve the translation against obstacles, and the resulting movement will be automatically applied to the entity's transform.

// Please note that the actual collision resolution and grounding checks are performed by the physics simulation, so you might need to adjust the values based on your specific requirements and the physics settings of your scene.