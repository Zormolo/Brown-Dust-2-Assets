use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{ Path, PathBuf };
use std::process::{Command, Stdio};
use serde::{Deserialize};

use rayon::prelude::*;
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
    interaction: HashMap< String, String >
}


fn extract_assets() {
    let path = Path::new( "F:/Neowiz/Browndust2/Browndust2_10000001/BrownDust II_Data/Addressable" );

    let pool = ThreadPoolBuilder::new().num_threads( 30 ).build().unwrap();
    let paths: Vec<PathBuf> = fs::read_dir( path ).unwrap().map(|res| res.unwrap().path() ).collect();

    pool.install( || {
        paths.par_iter().for_each( |path| {
            extract_folder( path );
            print!( "\r");
        } );
    } );
}

fn extract_folder( file_path: &PathBuf ) {
    let output = Command::new( ".//asset_extractor//ArknightsStudioCLI.exe" )
        .arg( &file_path )
        .args( [ "-o", "./output", "-t", "tex2d,textAsset,audio", "--unity-version", "2022.3.22f1" ] )
        .stdout(Stdio::null() )
        .status();

    match output {
        Ok(_status) => {
            print!("Folder {} extracted!", file_path.display() );
        }
        Err(err) => {
            eprintln!("failed to run exe: {}", err);
        }
    }
}

fn clear_output_folder() {
    let full_path = format!( "{}/{}", "./", "output" );
    let path = Path::new( &full_path );
    if fs::exists( path ).unwrap() {
        fs::remove_dir_all( path ).unwrap();
        fs::create_dir( path ).expect( "folder couldnt be created" );
    }
}

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

fn make_repo_structur() {
    remove_asset_files();
    let file = File::open( Path::new( "src/json/folder_structure.json" ) ).expect("test");
    let reader = BufReader::new( file );
    let folders: Vec<Folder> = serde_json::from_reader( reader ).expect("rrrrrr");

    make_folder( "./assets", folders );
}

fn sort_char_spine( file_path: PathBuf, character_map: HashMap< String, String >, file_name: String ) {
    let char_id = &file_name[..10];
    let mut copy_path= "assets\\spine\\character\\".to_string();
    if character_map.contains_key( char_id ) {
       copy_path.push_str( character_map.get( char_id ).unwrap() );
       copy_path.push_str( "\\" );
    }
    fs::rename( file_path, format!("{}{}", copy_path, &file_name ) ).expect("Spine Charactere Error");
    return;
}

fn sort_skill_cutscene_spine( file_path: PathBuf, skill_cutscene_map: HashMap< String, String >, file_name: String ) {
    let skill_cutscene_id = &file_name[..20];
    let mut copy_path= "assets\\spine\\skill_cutscene\\".to_string();
    if skill_cutscene_map.contains_key( skill_cutscene_id ) {
       copy_path.push_str( skill_cutscene_map.get( skill_cutscene_id ).unwrap() );
       copy_path.push_str( "\\" );
    }
    fs::rename( file_path, format!("{}{}", copy_path, &file_name ) ).expect("Skill Cutscene Error");
    return;
}

fn sort_interaction_spine( file_path: PathBuf, interaction_map: HashMap< String, String >, file_name: String ) {
    let file_name_vec: Vec< &str > = file_name.split( "." ).collect();
    let file_name_stem = file_name_vec[ 0 ].to_string();
    let file_name_stem_vec: Vec< &str > = file_name_stem.split( "_" ).collect();
    let interaction_id = format!( "{}_{}", file_name_stem_vec[ 0 ], file_name_stem_vec[ 1 ] );
    let mut copy_path= "assets\\spine\\interaction\\".to_string();
    if interaction_map.contains_key( interaction_id.as_str() ) {
       copy_path.push_str( interaction_map.get( interaction_id.as_str() ).unwrap() );
       copy_path.push_str( "\\" );
    }
    fs::rename( file_path, format!("{}{}", copy_path, &file_name ) ).expect("Interaction Error");
    return;
}


fn sort_assets_into_repo() {
    let file = File::open( Path::new( "src/json/mapping.json" ) ).expect("test");
    let reader = BufReader::new( file );
    let mapping: Mapping = serde_json::from_reader( reader ).expect("rrrrrr");
    let path = ".\\output";

    let char_spine = Regex::new( r"(?im)^char[0-6][\d_]*\.(?:png|atlas|skel)" ).unwrap();
    let skill_cutscene_spine = Regex::new( r"(?im)^cutscene_char[\d_]*\.(?:png|atlas|skel)" ).unwrap();
    let interaction_spine = Regex::new( r"(?im)^illust_dating[\d_]*\.(?:png|atlas|skel)" ).unwrap();
    let costume_face = Regex::new( r"(?im)^illust_inven_char[\d_]*\.png" ).unwrap();
    let costume_icon = Regex::new( r"(?im)^icon_costume[\d_]*\.png" ).unwrap();

    for entry in fs::read_dir( path ).expect( "failed to find asset folder" ) {
        let entry = entry.expect( "failed to find folder entry!" );
        let file_name = entry.file_name().into_string().unwrap();

        if char_spine.is_match( file_name.as_str() ) {
            sort_char_spine( entry.path(), mapping.charactere.clone(), file_name );
            continue;
        }

        if skill_cutscene_spine.is_match( file_name.as_str() ) {
            sort_skill_cutscene_spine( entry.path(), mapping.skill_cutscene.clone(), file_name );
            continue;
        }

        if interaction_spine.is_match( &entry.file_name().to_string_lossy() ) {
            sort_interaction_spine( entry.path(), mapping.interaction.clone(), file_name );
            continue;
        }

        if costume_face.is_match( &file_name ) {
            let file_name_stem_vec: Vec< &str > = file_name.split( "_" ).collect();
            let new_file_name = format!( "{}_{}_{}", file_name_stem_vec[ 0 ], file_name_stem_vec[ 1 ], file_name_stem_vec[ 2 ] );
            fs::rename( entry.path(), format!("{}{}.png", "assets\\ui\\costume_face\\", new_file_name ) ).expect("");
            continue;
        }

        if costume_icon.is_match( &file_name ) {
            let file_name_stem_vec: Vec< &str > = file_name.split( "_" ).collect();
            let new_file_name = format!( "{}_{}", file_name_stem_vec[ 0 ], file_name_stem_vec[ 1 ] );
            fs::rename( entry.path(), format!("{}{}.png", "assets\\ui\\costume_icon\\", new_file_name ) ).expect("");
            continue;
        }
    }
}

fn main() {
    clear_output_folder();

    extract_assets();
    println!( "\nExtraction completed!!!" );

    make_repo_structur();
    sort_assets_into_repo();
}
