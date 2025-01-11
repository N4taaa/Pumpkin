pub mod packet {
    include!(concat!(env!("OUT_DIR"), "/packet.rs"));
}

pub mod screen {
    include!(concat!(env!("OUT_DIR"), "/screen.rs"));
}

pub mod particle {
    include!(concat!(env!("OUT_DIR"), "/particle.rs"));
}

pub mod sound {
    include!(concat!(env!("OUT_DIR"), "/sound.rs"));
    include!(concat!(env!("OUT_DIR"), "/sound_category.rs"));
}

pub mod chunk {
    include!(concat!(env!("OUT_DIR"), "/noise_parmeter.rs"));
    include!(concat!(env!("OUT_DIR"), "/chunk_status.rs"));
}

pub mod game_event {
    include!(concat!(env!("OUT_DIR"), "/game_event.rs"));
}

pub mod entity {
    include!(concat!(env!("OUT_DIR"), "/entity_type.rs"));
    include!(concat!(env!("OUT_DIR"), "/entity_pose.rs"));
}

pub mod world {
    include!(concat!(env!("OUT_DIR"), "/world_event.rs"));
}

pub mod scoreboard {
    include!(concat!(env!("OUT_DIR"), "/scoreboard_slot.rs"));
}
