use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{ Path, PathBuf };
use std::process::{Command, Stdio};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use serde::{Deserialize};
use rayon::ThreadPoolBuilder;

#[ derive( Deserialize, Debug ) ]
struct Folder {
    name: String,
    folder_list: Option<Vec<Folder>>
}

#[ derive( Deserialize, Debug ) ]
struct Mapping {
    charactere: HashMap< String, String >,
    skill_cutscene: HashMap< String, String >,
    interaction: HashMap< String, String >,
    npc: HashMap< String, String >,
    light_novel_talk: HashMap< String, String >
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn extract_assets() {
    let path = Path::new( "F:/Neowiz/Browndust2/Browndust2_10000001/BrownDust II_Data/Addressable" );

    let pool = ThreadPoolBuilder::new().num_threads( 30 ).build().unwrap();
    let paths: Vec<PathBuf> = fs::read_dir( path ).unwrap().map(|res| res.unwrap().path() ).collect();

    let total_folders = paths.len();
    let processed_folders = Arc::new( AtomicUsize::new( 0 ) );

    pool.scope( | s | {
        for folder_path in paths {
            let processed_clone = Arc::clone( &processed_folders );

            s.spawn( move | _ | {
                extract_folder( &folder_path );
                processed_clone.fetch_add(1, Ordering::Relaxed );
                print!( "{}Folders {} / {} extracted!\r", "\x1b[2K", processed_clone.load( Ordering::Relaxed ), total_folders );
            } );
        }

        while processed_folders.load( Ordering::Relaxed ) < total_folders {
            let count = processed_folders.load( Ordering::Relaxed );
            print!( "\x1b[2KFolders {} / {} extracted!\r", count, total_folders );
            std::io::Write::flush( &mut std::io::stdout() ).unwrap();
        }
    } );
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn extract_folder( file_path: &PathBuf ) {
    let output = Command::new( "asset_extractor//3rd_party//ArknightsStudioCLI//ArknightsStudioCLI.exe" )
        .arg( &file_path )
        .args( [ "-o", "./output", "-t", "tex2d,textAsset", "--unity-version", "2022.3.22f1" ] )
        .stdout(Stdio::null() )
        .status();

    match output {
        Ok(_status) => {
            // println!("{}", _status);
        }
        Err(err) => {
            eprintln!("failed to run exe: {}", err);
        }
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn clear_output_folder() {
    let path = Path::new( "output" );
    if fs::exists( path ).unwrap() {
        fs::remove_dir_all( path ).unwrap();
        fs::create_dir( path ).expect( "folder couldnt be created" );
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn make_folder( base_path: &str, subfolder: Vec<Folder> ) {
    for folder in subfolder {
        let path_string = format!("{}/{}", base_path, folder.name );
        let path = Path::new( &path_string );
        if !fs::exists( path ).expect("folder path exists already!") {
            fs::create_dir( path ).expect( "folder couldnt be created" );
        }
        if let Some( list ) = folder.folder_list {
            make_folder( &path_string, list );
        }
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn remove_asset_files() {
    let folders = vec![
        "spine",
        "ui"
    ];

    for folder in folders {
        let full_path = format!( "{}/{}", "assets/", folder );
        let path = Path::new( &full_path );
        if fs::exists( path ).unwrap() {
            fs::remove_dir_all( path ).unwrap();
        }
    };
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn make_repo_structur() {
    remove_asset_files();
    let file = File::open( Path::new( "asset_extractor/json/folder_structure.json" ) ).expect("test");
    let reader = BufReader::new( file );
    let folders: Vec<Folder> = serde_json::from_reader( reader ).expect("rrrrrr");

    make_folder( "assets", folders );
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn sort_char_spine( file_path: PathBuf, character_map: HashMap< String, String >, file_name: String ) {
    let char_id = &file_name[..10];
    let mut copy_path= "assets\\spine\\character\\".to_string();
    if character_map.contains_key( char_id ) {
       copy_path.push_str( character_map.get( char_id ).unwrap() );
       copy_path.push_str( "\\" );
       if !fs::exists( &copy_path ).unwrap() {
            println!( "Could not find: {}", copy_path );
            copy_path = "assets\\spine\\character\\".to_string();
       }
    }
    fs::rename( file_path, format!("{}{}", copy_path, &file_name ) ).expect("Spine Charactere Error");
    return;
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn sort_skill_cutscene_spine( file_path: PathBuf, skill_cutscene_map: HashMap< String, String >, file_name: String ) {
    let skill_cutscene_id = &file_name[..19];
    if skill_cutscene_id == "cutscene_char061303" {
        fs::remove_file( &file_path ).expect( "Could not delete duplicate spine skill_cutscene" );
        return;
    }
    let mut copy_path= "assets\\spine\\skill_cutscene\\".to_string();
    if skill_cutscene_map.contains_key( skill_cutscene_id ) {
       copy_path.push_str( skill_cutscene_map.get( skill_cutscene_id ).unwrap() );
       copy_path.push_str( "\\" );
       if !fs::exists( &copy_path ).unwrap() {
            println!( "Could not find: {}", copy_path );
            copy_path = "assets\\spine\\skill_cutscene\\".to_string();
       }
    }
    fs::rename( file_path, format!("{}{}", copy_path, &file_name ) ).expect("Skill Cutscene Error");
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn sort_interaction_spine( file_path: PathBuf, interaction_map: HashMap< String, String >, file_name: String ) {
    let file_name_vec: Vec< &str > = file_name.split( "." ).collect();
    let file_name_stem = file_name_vec[ 0 ].to_string();
    let file_name_stem_vec: Vec< &str > = file_name_stem.split( "_" ).collect();
    let interaction_id = format!( "{}_{}", file_name_stem_vec[ 0 ], file_name_stem_vec[ 1 ] );
    let mut copy_path= "assets\\spine\\interaction\\".to_string();
    if interaction_map.contains_key( interaction_id.as_str() ) {
       copy_path.push_str( interaction_map.get( interaction_id.as_str() ).unwrap() );
       copy_path.push_str( "\\" );
       if !fs::exists( &copy_path ).unwrap() {
            println!( "Could not find: {}", copy_path );
            copy_path = "assets\\spine\\interaction\\".to_string();
       }
    }
    fs::rename( file_path, format!("{}{}", copy_path, &file_name ) ).expect("Interaction Error");
    return;
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn sort_npc_spine( file_path: PathBuf, npc_map: HashMap< String, String >, file_name: String ) {
    let file_name_vec: Vec< &str > = file_name.split( "." ).collect();
    let npc_id = file_name_vec[ 0 ].to_string();
    let mut copy_path = "assets\\spine\\npc\\".to_string();
    if npc_map.contains_key( npc_id.as_str() ) {
       copy_path.push_str( npc_map.get( npc_id.as_str() ).unwrap() );
       copy_path.push_str( "\\" );
       if !fs::exists( &copy_path ).unwrap() {
            println!( "Could not find: {}", copy_path );
            copy_path = "assets\\spine\\npc\\".to_string();
       }
    }
    fs::rename( file_path, format!("{}{}", &copy_path, &file_name ) ).expect( "NPC Spine Error" );
    return;
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn sort_light_novel_talk_spine( file_path: PathBuf, light_novel_talk_map: HashMap< String, String >, file_name: String ) {
    let file_name_vec: Vec< &str > = file_name.split( "." ).collect();
    let file_name_stem = file_name_vec[ 0 ].to_string();
    let file_name_stem_vec: Vec< &str > = file_name_stem.split( "_" ).collect();
    let light_novel_talk_id = format!( "{}_{}", file_name_stem_vec[ 0 ], file_name_stem_vec[ 1 ] );
    let mut copy_path = "assets\\spine\\light_novel_talk\\".to_string();
    if light_novel_talk_map.contains_key( light_novel_talk_id.as_str() ) {
       copy_path.push_str( light_novel_talk_map.get( light_novel_talk_id.as_str() ).unwrap() );
       copy_path.push_str( "\\" );
       if !fs::exists( &copy_path ).unwrap() {
            println!( "Could not find: {}", copy_path );
            copy_path = "assets\\spine\\light_novel_talk\\".to_string();
       }
    }
    fs::rename( file_path, format!("{}{}", &copy_path, &file_name ) ).expect( "Light Novel Talk Spine Error" );
    return;
}



///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn sort_assets_into_repo() {
    let file = File::open( Path::new( "asset_extractor/json/mapping.json" ) ).expect("test");
    let reader = BufReader::new( file );
    let mapping: Mapping = serde_json::from_reader( reader ).expect("rrrrrr");

    let char_spine = Regex::new( r"(?im)^char[0-6][\d_]*\.(?:png|atlas|skel)" ).unwrap();
    let skill_cutscene_spine = Regex::new( r"(?im)^cutscene_char[\d_]*\.(?:png|atlas|skel)" ).unwrap();
    let interaction_spine = Regex::new( r"(?im)^illust_dating[\d_]*\.(?:png|atlas|skel)" ).unwrap();
    let npc_spine = Regex::new( r"^npc[_ellin|\d]*\.(?:png|atlas|skel)" ).unwrap();
    let light_novel_talk_spine = Regex::new( r"^illust_talk[_\d]*\.(?:png|atlas|skel)" ).unwrap();

    let costume_face = Regex::new( r"(?im)^illust_inven_char[\d_]*\.png" ).unwrap();
    let costume_icon = Regex::new( r"(?im)^icon_costume[\d_]*\.png" ).unwrap();
    let buff_icon_atlas = Regex::new( r"(?im)^sactx\S+-BuffIcon\S+\.png" ).unwrap();

    for entry in fs::read_dir( "output" ).expect( "failed to find asset folder" ) {
        let entry = entry.expect( "failed to find folder entry!" );
        let file_name = entry.file_name().into_string().unwrap();

        if file_name.contains( "#" ) {
            continue;
        }

        if char_spine.is_match( &file_name ) {
            sort_char_spine( entry.path(), mapping.charactere.clone(), file_name );
            continue;
        }

        if skill_cutscene_spine.is_match( &file_name ) {
            sort_skill_cutscene_spine( entry.path(), mapping.skill_cutscene.clone(), file_name );
            continue;
        }

        if interaction_spine.is_match( &file_name ) {
            sort_interaction_spine( entry.path(), mapping.interaction.clone(), file_name );
            continue;
        }

        if npc_spine.is_match( &file_name ) {
            sort_npc_spine( entry.path(), mapping.npc.clone(), file_name );
            continue;
        }

        if light_novel_talk_spine.is_match( &file_name ) {
            sort_light_novel_talk_spine( entry.path(), mapping.light_novel_talk.clone(), file_name );
            continue;
        }

        if costume_face.is_match( &file_name ) {
            let file_name_stem_vec: Vec< &str > = file_name.split( "_" ).collect();
            let new_file_name = format!( "{}_{}_{}", file_name_stem_vec[ 0 ], file_name_stem_vec[ 1 ], file_name_stem_vec[ 2 ] );
            fs::rename( entry.path(), format!( "{}{}.png", "assets\\ui\\costume_face\\", new_file_name ) ).expect("");
            continue;
        }

        if costume_icon.is_match( &file_name ) {
            let file_name_stem_vec: Vec< &str > = file_name.split( "_" ).collect();
            let new_file_name = format!( "{}_{}", file_name_stem_vec[ 0 ], file_name_stem_vec[ 1 ] );
            fs::rename( entry.path(), format!( "{}{}.png", "assets\\ui\\costume_icon\\", new_file_name ) ).expect("");
            continue;
        }

        if buff_icon_atlas.is_match( &file_name ) {
            fs::copy( entry.path(), format!( "{}{}", "assets\\ui\\skill_icons\\", file_name ) ).expect("");
            continue;
        }
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    // clear_output_folder();

    // extract_assets();
    // println!( "\nExtraction completed!!!" );

    // make_repo_structur();
    sort_assets_into_repo();

    fixing_shit();
}

fn fixing_shit() {
    extract_skill_icons();
}

use skill_icon_extractor::add;
fn extract_skill_icons() {
    println!( "{}", add(2, 2) )
}
