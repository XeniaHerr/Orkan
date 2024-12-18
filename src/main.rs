mod orkan;

use orkan::draw_utils::Renderer;
use orkan::search_element;
use orkan::window;

use smithay_client_toolkit::compositor::CompositorState;
use smithay_client_toolkit::output::OutputState;
use smithay_client_toolkit::registry::RegistryState;
use smithay_client_toolkit::seat::SeatState;
use smithay_client_toolkit::shell::wlr_layer::Layer;
use smithay_client_toolkit::shell::wlr_layer::LayerShell;

use smithay_client_toolkit::shell::WaylandSurface;
use smithay_client_toolkit::shm::slot::SlotPool;
use smithay_client_toolkit::shm::Shm;
use wayland_client;
use wayland_client::globals;
use wayland_client::Connection;
use smithay_client_toolkit::shell::wlr_layer::Anchor;

use fontconfig::Fontconfig;


use clap::Parser;
use orkan::config::Config;

use rusttype::Font;
use std::fs;




fn main() {

    println!("Staring Orkan");

    let config : Config = Config::parse();


    println!("fg_color {}", config.fontcolor.to_string());


    let fontname = config.font.clone();

    println!("Fontname: {}", fontname);

    let fc = Fontconfig::new().unwrap();

    let fontpath = fc.find(fontname.as_str(), None).unwrap();
    println!("Font path: {:?}", fontpath.path.to_str());

    let font_data : Vec<u8> = fs::read(fontpath.path).unwrap();

    let font = Font::try_from_vec(font_data).unwrap();

    let conn : Connection = Connection::connect_to_env().unwrap();


    

    let (globals, mut event_queue) = globals::registry_queue_init(&conn).unwrap();


    let qh   = event_queue.handle();

    let compositor = CompositorState::bind(&globals, &qh).expect("Compositor is not available");

    let layer_shell = LayerShell::bind(&globals, &qh).expect("Layer shell is not supported");

    let shm = Shm::bind(&globals, &qh).expect("Shm is not available");

    let surface = compositor.create_surface(&qh);

    let layer = layer_shell.create_layer_surface(&qh, surface, Layer::Top, Some("OrkanWindow"), None);

    layer.set_anchor(Anchor::TOP|Anchor::LEFT|Anchor::RIGHT);
    layer.set_keyboard_interactivity(smithay_client_toolkit::shell::wlr_layer::KeyboardInteractivity::Exclusive);
    layer.set_size(0, config.height as u32);
    layer.set_margin(config.top_margin , config.left_margin, 0, config.left_margin);
    layer.commit();

    let renderer : Renderer = Renderer::new(font,&config, 0, config.height as u32);

    // This pool has some stupid default values, because i don't know how big the screen will be
    let pool = SlotPool::new(1920* 1080 *4, &shm).expect("Failed to create pool");

    println!("Setup done");

    let mut window = window::OrkanWindow {
        registry_state : RegistryState::new(&globals),
        seat_state : SeatState::new(&globals, &qh),
        output_state : OutputState::new(&globals, &qh),
        shm : shm, 


        exists : true,

        pool : pool,

        renderer : renderer,
        keyboard : None,


        buffer : None,

        
        valid_elements : Vec::new(),

        data : search_element::Searcher::binary_searcher(),

        has_keyboard : false,
        layer_surface : layer,

        shift : None,

        highlighted_pos : 0,

        first_configure : true,

        padding_rel: 0.1,
        padding_abs: 0,


        need_update : true,


    };

    println!("Window created");
    loop {
        event_queue.blocking_dispatch(&mut window).unwrap();

        if !window.exists {
            break;
        }

    }


}

