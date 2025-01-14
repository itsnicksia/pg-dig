#![allow(unused_variables)]
#![allow(unsafe_code)]
#![allow(dead_code)]
use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use pg_dig_server::postgres::bindings::PQfinish;
use pg_dig_server::postgres::connection::connect;
use pg_dig_server::postgres::replication::{read_message, start_replication};
use std::sync::mpsc::*;
use std::sync::Mutex;
use std::thread;
use pg_dig_server::postgres::xlog_message::XLogMessage;

const IMAGE_WIDTH: u32 = 512;
const IMAGE_HEIGHT: u32 = 512;

#[derive(Resource)]
struct ReceiveChannel {
    receiver: Mutex<Receiver<XLogMessage>>,
}

/// Store the image handle that we will draw to, here.
#[derive(Resource)]
struct MyProcGenImage(Handle<Image>);

const LOCAL_CONNECTION_STRING: &str = "host=localhost user=postgres dbname=postgres password=postgres replication=database";
fn main() {
    let (tx, rx): (Sender<XLogMessage>, Receiver<XLogMessage>) = channel();

    let consumer_handle = thread::spawn(move || {
        unsafe {
            let conn = connect(LOCAL_CONNECTION_STRING);
            start_replication(conn).unwrap();

            loop {
                match read_message(conn) {
                    Ok(message) => {
                        println!("debug: {:#?}", message);
                        tx.send(message).unwrap();
                        break;
                    },
                    Err(e) => {
                        println!("failed to read message: {}", e);
                        break;
                    }
                }
            }

            PQfinish(conn);
        }
    });

    //start_renderer(rx);
    start_dummy_consumer(rx);
}

fn start_dummy_consumer(rx: Receiver<XLogMessage>) {
    loop {
        let _ = rx.recv();
    }
}

fn start_renderer(rx: Receiver<XLogMessage>) {
    App::new()
        .insert_resource(ReceiveChannel { receiver: Mutex::new(rx) })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, draw)
        .run();
}

fn draw(
    handle: Res<MyProcGenImage>,
    mut images: ResMut<Assets<Image>>,
    receiver_channel: Res<ReceiveChannel>,
    mut draw_color: Local<Color>
) {
    let receiver = match receiver_channel.receiver
        .try_lock() {
        Ok(receiver) => receiver,
        Err(_) => panic!("failed to lock receiver")
    };

    let pixels = IMAGE_WIDTH * IMAGE_HEIGHT;
    let image = images.get_mut(&handle.0).expect("Image not found");

    match receiver.try_recv() {
        Ok(message) => {
            println!("message: {:#?}", message);
            let block_numbers = message.get_block_numbers();
            match block_numbers.first() {
                Some(block_number) => {
                    if *block_number <= IMAGE_WIDTH * IMAGE_HEIGHT {
                        *draw_color = Color::linear_rgb(255f32, 0f32, 0f32);
                        let (x, y) = (*block_number % IMAGE_WIDTH, *block_number / IMAGE_WIDTH);
                        println!("writing at ({}, {})", x, y);
                        image
                            .set_color_at(x, y, *draw_color)
                            .unwrap();
                    }
                },
                None => {}
            }

        }
        Err(_) => {}
    }
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // spawn a camera
    commands.spawn(Camera2d);

    // create an image that we are going to draw into
    let image = Image::new_fill(
        // 2D image of size 256x256
        Extent3d {
            width: IMAGE_WIDTH,
            height: IMAGE_HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // Initialize it with a beige color
        &(css::BLACK.to_u8_array()),
        // Use the same encoding as the color we set
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    // add it to Bevy's assets, so it can be used for rendering
    // this will give us a handle we can use
    // (to display it in a sprite, or as part of UI, etc.)
    let handle = images.add(image);

    // create a sprite entity using our image
    commands.spawn(Sprite::from_image(handle.clone()));
    commands.insert_resource(MyProcGenImage(handle));
}
//
// let record_block_headers.iter().map(|block_header| {
// let fork_number = block_header.fork_flags >> 4;
// Info {
// block_number: block_header.block_number,
// table_name: match block_header.rel_file_locator {
// Some(loc) => loc.rel_number.to_string(),
// None => "unknown".to_string(),
// },
// fork_name: match fork_number {
// 0 => "main",
// 1 => "fsm",
// 2 => "vis",
// 3 => "init",
// _ => panic!("invalid fork number: {}", fork_number),
// }.to_string()
// }
// }
//
//
// todo!();
