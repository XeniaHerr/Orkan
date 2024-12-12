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

use rusttype::Font;
use std::fs;


#[allow(dead_code)]
struct Config {

    left_padding : Option<f32>,

    top_padding : Option<f32>,

    font : Option<String>,

    background_color : [u8; 4],


    text_color : [u8; 4],
}



fn read_config() -> Config {


    let mut _configs = Config {
        left_padding: Some(0.1),
        top_padding: Some(0.2),

        font: None,
        background_color: [0x00, 0x00, 0x00, 0x00],
        text_color: [0xff, 0xff, 0xff, 0xff],
    };
    let arguments = std::env::args().collect::<Vec<String>>();

    let mut args = arguments.iter();


    while let Some(arg) = args.next() {

        match arg.as_str() {

            "--top-margin" => {

//                _configs.top_padding = Some(args.next().unwrap().parse::<f32>().unwrap());
                _configs.top_padding =  Some(arg.split("=").collect::<Vec<&str>>()[1].parse::<f32>().unwrap());

                    if _configs.top_padding.unwrap() > 0.9 {
                        _configs.top_padding = Some (0.9);
                    }
            }
            "--left-margin" => {

                _configs.left_padding = Some(arg.split("=").collect::<Vec<&str>>()[1].parse::<f32>().unwrap());

                    if _configs.left_padding.unwrap() > 0.9 {
                        _configs.left_padding = Some (0.9);
                    }


            }

            _ => {}
        }
    }




    return _configs;

}



fn main() {

    println!("Staring Orkan");

    let fontname = "Mononoki Nerd Font".to_string();

    let fc = Fontconfig::new().unwrap();

    let fontpath = fc.find(fontname.as_str(), None).unwrap();
    println!("Font path: {:?}", fontpath.path.to_str());

    let font_data : Vec<u8> = fs::read(fontpath.path).unwrap();

    let font = Font::try_from_vec(font_data).unwrap();

    let conn : Connection = Connection::connect_to_env().unwrap();

    let _ = read_config();

    

    let (globals, mut event_queue) = globals::registry_queue_init(&conn).unwrap();


    let qh   = event_queue.handle();

    let compositor = CompositorState::bind(&globals, &qh).expect("Compositor is not available");

    let layer_shell = LayerShell::bind(&globals, &qh).expect("Layer shell is not supported");

    let shm = Shm::bind(&globals, &qh).expect("Shm is not available");

    let surface = compositor.create_surface(&qh);

    let layer = layer_shell.create_layer_surface(&qh, surface, Layer::Top, Some("OrkanWindow"), None);

    layer.set_anchor(Anchor::TOP|Anchor::LEFT|Anchor::RIGHT);
    layer.set_keyboard_interactivity(smithay_client_toolkit::shell::wlr_layer::KeyboardInteractivity::Exclusive);
    layer.set_size(0, 20);
    layer.set_margin(10, 0, 0, 50);
    layer.commit();

    let renderer : Renderer = Renderer::new(font, 0, 20);

    let pool = SlotPool::new(400* 20 *4, &shm).expect("Failed to create pool");

    println!("Setup done");

    let mut window = window::OrkanWindow {
        registry_state : RegistryState::new(&globals),
        seat_state : SeatState::new(&globals, &qh),
        output_state : OutputState::new(&globals, &qh),
        shm : shm, 

        exists : true,

        //font : font,
        pool : pool,
        //width : 400,
        //height : 20,

        renderer : renderer,
        keyboard : None,

 //       compositor_state : compositor,

        buffer : None,

        //cur_search : Vec::new(),
        
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

