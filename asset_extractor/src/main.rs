use regex::{Regex};
use std::collections::HashMap;
use std::fs::{self, DirEntry, File};
use std::io::{BufReader};
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
    let path = Path::new( "F:\\Gamfs_BrownDust II" );

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

fn extract_folder( file_path: &Path ) {
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

fn sort_char_spine( file_path: &str, character_map: HashMap< String, String >, file_name: &str ) {
    let char_id = file_name[..10].to_string();
    let err_msg = "Spine Charactere Error";
    move_file( &file_path, file_name, "assets\\spine\\character\\", character_map, char_id, err_msg );
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn sort_skill_cutscene_spine( file_path: &str, skill_cutscene_map: HashMap< String, String >, file_name: &str ) {
    let skill_cutscene_id = file_name[..19].to_string();
    if skill_cutscene_id == "cutscene_char061303" {
        fs::remove_file( &file_path ).expect( "Could not delete duplicate spine skill_cutscene" );
        return;
    }
    let err_msg = "Skill Cutscene Error";
    move_file( &file_path, file_name, "assets\\spine\\skill_cutscene\\", skill_cutscene_map, skill_cutscene_id, err_msg ); 
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn sort_interaction_spine( file_path: &str, interaction_map: HashMap< String, String >, file_name: &str ) {
    let file_name_vec: Vec< &str > = file_name.split( "." ).collect();
    let file_name_stem = file_name_vec[ 0 ].to_string();
    let file_name_stem_vec: Vec< &str > = file_name_stem.split( "_" ).collect();
    let interaction_id = format!( "{}_{}", file_name_stem_vec[ 0 ], file_name_stem_vec[ 1 ] );
    let err_msg = "Interaction Error";
    move_file( &file_path, file_name, "assets\\spine\\interaction\\", interaction_map, interaction_id, err_msg );
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn sort_npc_spine( file_path: &str, npc_map: HashMap< String, String >, file_name: &str ) {
    let file_name_vec: Vec< &str > = file_name.split( "." ).collect();
    let npc_id = file_name_vec[ 0 ].to_string();
    let err_msg = "NPC Spine Error";
    move_file( &file_path, file_name, "assets\\spine\\npc\\", npc_map, npc_id, err_msg );
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn sort_light_novel_talk_spine( file_path: &str, light_novel_talk_map: HashMap< String, String >, file_name: &str ) {
    let file_name_vec: Vec< &str > = file_name.split( "." ).collect();
    let file_name_stem = file_name_vec[ 0 ].to_string();
    let file_name_stem_vec: Vec< &str > = file_name_stem.split( "_" ).collect();
    let light_novel_talk_id = format!( "{}_{}", file_name_stem_vec[ 0 ], file_name_stem_vec[ 1 ] );
    let err_msg = "Light Novel Talk Spine Error";
    move_file( file_path, file_name, "assets\\spine\\light_novel_talk\\", light_novel_talk_map, light_novel_talk_id, err_msg );
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn move_file( file_path: &str, file_name: &str, base_path: &str, map: HashMap< String, String >, map_key: String, err_msg: &str ) {
    let mut copy_path = base_path.to_string();
    if map.contains_key( map_key.as_str() ) {
       copy_path.push_str( map.get( map_key.as_str() ).unwrap() );
       copy_path.push_str( "\\" );
       if !fs::exists( &copy_path ).unwrap() {
            println!( "Could not find: {}", copy_path );
            copy_path = base_path.to_string();
       }
    }
    if file_path.contains( "archive\\" ) {
        fs::copy( file_path, format!("{}{}", &copy_path, &file_name ) ).expect( err_msg );
        return;
    }
    fs::rename( file_path, format!("{}{}", &copy_path, &file_name ) ).expect( err_msg );
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn sort_assets_into_repo() {
    let file = File::open( Path::new( "asset_extractor/json/mapping.json" ) ).expect("test");
    let reader = BufReader::new( file );
    let mapping: Mapping = serde_json::from_reader( reader ).expect("rrrrrr");

    let char_spine = Regex::new( r"(?im)^char[0-6][\d_c]*\.(?:png|atlas|skel)" ).unwrap();
    let skill_cutscene_spine = Regex::new( r"(?im)^cutscene_char[\d_a]*\.(?:png|atlas|skel)" ).unwrap();
    let interaction_spine = Regex::new( r"(?im)^illust_dating[\d_]*\.(?:png|atlas|skel)" ).unwrap();
    let npc_spine = Regex::new( r"^npc[_ellin|\d]*\.(?:png|atlas|skel)" ).unwrap();
    let light_novel_talk_spine = Regex::new( r"^illust_talk[_\d]*\.(?:png|atlas|skel)" ).unwrap();

    let costume_face = Regex::new( r"(?im)^illust_inven_char[\d_c]*\.png" ).unwrap();
    let costume_skill_face = Regex::new( r"(?im)^illust_skill_char[\d_]*\.png" ).unwrap();
    let costume_icon = Regex::new( r"(?im)^icon_costume[\d_]*\.png" ).unwrap();
    let buff_icon_atlas = Regex::new( r"(?im)^sactx\S+-BuffIcon\S+\.png" ).unwrap();
    let skill_cutscene_background = Regex::new( r"(?im)back[\d\. ]+" ).unwrap();

    let wallpapers = Regex::new( r"(?im)^bg_idcard_bg[\d_a-z]*\.png" ).unwrap();

    let output_folder_files_path = fs::read_dir( "output" ).expect( "failed to find asset folder" );
    let archive_folder_files_path = fs::read_dir( "archive" ).expect( "failed to find archive folder" );
    let mut assets_files: Vec< Result< DirEntry, std::io::Error > > = Vec::new();
    collect_files( Path::new( "output\\Assets" ), &mut assets_files );
    let mut file_paths: Vec<_ > = output_folder_files_path.chain( archive_folder_files_path ).collect();
    file_paths.append( &mut assets_files );

    for entry in file_paths {
        let entry = entry.expect( "failed to find folder entry!" );
        let file_name_temp = entry.file_name();
        let mut file_name = file_name_temp.to_str().unwrap();
        let entry_path = entry.path();
        let mut path = entry_path.to_str().unwrap();

        if file_name.contains( "#" ) {
            let cleaned_path: Vec<&str> = path.split( "_#" ).collect();
            fs::rename( path, cleaned_path[0] ).expect( "" );
            path = cleaned_path[0];
            let file_name_vec: Vec< &str > = file_name.split( "_#" ).collect();
            file_name = file_name_vec[0];
        }

        if char_spine.is_match( &file_name ) {
            sort_char_spine( path, mapping.charactere.clone(), file_name );
            continue;
        }

        if skill_cutscene_spine.is_match( &file_name ) {
            
            sort_skill_cutscene_spine( path, mapping.skill_cutscene.clone(), file_name );
            continue;
        }

        if interaction_spine.is_match( &file_name ) {
            sort_interaction_spine( path, mapping.interaction.clone(), file_name );
            continue;
        }

        if npc_spine.is_match( &file_name ) {
            sort_npc_spine( path, mapping.npc.clone(), file_name );
            continue;
        }

        if light_novel_talk_spine.is_match( &file_name ) {
            sort_light_novel_talk_spine( path, mapping.light_novel_talk.clone(), file_name );
            continue;
        }

        if costume_face.is_match( &file_name ) {
            let new_file_name: String;
            if file_name.contains( "_c." ) {
                new_file_name = file_name.replace( ".png", "" );
            } else {
                let file_name_stem_vec: Vec< &str > = file_name.split( "_" ).collect();
                new_file_name = format!( "{}_{}_{}", file_name_stem_vec[ 0 ], file_name_stem_vec[ 1 ], file_name_stem_vec[ 2 ] );
            }
            fs::rename( path, format!( "{}{}.png", "assets\\ui\\costume_face\\", new_file_name ) ).expect("");
            continue;
        }

        if costume_skill_face.is_match( &file_name ) {
            let file_name_stem_vec: Vec< &str > = file_name.split( "_" ).collect();
            let new_file_name = format!( "{}_{}_{}", file_name_stem_vec[ 0 ], file_name_stem_vec[ 1 ], file_name_stem_vec[ 2 ] );
            fs::rename( path, format!( "{}{}.png", "assets\\ui\\costume_skill_face\\", new_file_name ) ).expect("");
            continue;
        }

        if costume_icon.is_match( &file_name ) {
            let file_name_stem_vec: Vec< &str > = file_name.split( "_" ).collect();
            let new_file_name = format!( "{}_{}", file_name_stem_vec[ 0 ], file_name_stem_vec[ 1 ] );
            fs::rename( path, format!( "{}{}.png", "assets\\ui\\costume_icon\\", new_file_name ) ).expect("");
            continue;
        }

        if buff_icon_atlas.is_match( &file_name ) {
            fs::copy( path, format!( "{}{}", "assets\\ui\\skill_icons\\", file_name ) ).expect("");
            continue;
        }

        if skill_cutscene_background.is_match( &file_name ) {
            fs::rename( path, format!( "{}{}", "assets\\ui\\skill_cutscene_background\\", file_name ) ).expect("");
            continue;
        }

        if wallpapers.is_match( &file_name ) {
            if path.contains( "archive\\" ) {
                fs::copy( path, format!( "{}{}", "assets\\ui\\wallpapers\\", file_name ) ).expect("");
                continue;
            }
            fs::rename( path, format!( "{}{}", "assets\\ui\\wallpapers\\", file_name ) ).expect("");
            continue;
        }
    }
}

fn collect_files(dir: &Path, files: &mut Vec< Result< DirEntry, std::io::Error > >) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let e = entry;
            let t = e.as_ref().unwrap();
            let path = t.path();

            if path.is_dir() {
                collect_files(&path, files); // recurse into subfolder
            } else {
                files.push( e );
            }
        }
    }
}


fn prefix() {
    fs::remove_dir_all( "output\\Assets\\AddressableResources\\BundleCommon\\SkeletonData\\cutscene_char004201_1" ).expect( "" );
    fs::rename( "output\\Assets\\AddressableResources\\BundleCommon\\UI_DynamicImg\\IllustInventory_1\\Censorship\\illust_inven_char000502_98.png", "output\\illust_inven_char000502_c.png" ).expect( "" );
    fs::rename( "output\\Assets\\AddressableResources\\BundleCommon\\UI_DynamicImg\\IllustInventory_1\\Censorship\\illust_inven_char066802_121.png", "output\\illust_inven_char066802_c.png" ).expect( "" );
    fs::rename( "output\\Assets\\AddressableResources\\BundleCommon\\UI_DynamicImg\\IllustInventory_1\\Censorship\\illust_inven_char101101_67.png", "output\\illust_inven_char101101_c.png" ).expect( "" );
    fs::rename( "output\\Assets\\AddressableResources\\BundleCommon\\UI_DynamicImg\\IllustInventory_1\\Censorship\\illust_inven_char101102_25.png", "output\\illust_inven_char101102_c.png" ).expect( "" );
}
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    clear_output_folder();

    extract_assets();
    println!( "\nExtraction completed!!!" );

    prefix();

    make_repo_structur();
    sort_assets_into_repo();

    fixing_shit();
}

fn fixing_shit() {
    fix_last_hope_loen();
    fix_manager_gray();
    fix_cursed_celia();
    fix_b_rank_idol_helena();
    fix_rising_star_helena();
    fix_costume_icon_ids();
    fix_eff_wallpapers();
    // extract_skill_icons();
}

fn fix_last_hope_loen() {
    let source = "assets\\spine\\npc\\Loen\\";
    let spine_files: Vec<PathBuf> = fs::read_dir( source ).unwrap().map(|res| res.unwrap().path() ).collect();
    for file in spine_files {
        let mut target = "assets\\spine\\character\\Loen\\Last_Hope\\".to_string();
        target.push_str( "char003201." );
        let file_name = file.file_name().unwrap().to_string_lossy();
        let file_name_vec: Vec< &str > = file_name.split( "." ).collect();
        target.push_str( file_name_vec[ 1 ] );

        if file_name_vec[ 1 ] == "atlas" {
            let mut atlas_content = fs::read_to_string( &file ).expect( "" );
            atlas_content = atlas_content.replace( "npc300501" , "char003201" );
            fs::write( target, atlas_content ).expect( "" );
            continue;
        }
        fs::copy( &file, &target ).expect( "" );
    }
}

fn fix_manager_gray() {
    let source = "assets\\spine\\character\\Gray\\B-Rank_Manager\\";
    let spine_files: Vec<PathBuf> = fs::read_dir( source ).unwrap().map(|res| res.unwrap().path() ).collect();
    for file in spine_files {
        let file_name = file.file_name().unwrap().to_string_lossy();
        let file_name_vec: Vec< &str > = file_name.split( "." ).collect();

        let target = file.to_str().unwrap().replace( ".skel.", "." );

        if file_name_vec[ 2 ] == "atlas" {
            let mut atlas_content = fs::read_to_string( &file ).expect( "" );
            atlas_content = atlas_content.replace( "char000402.skel" , "char000402" );
            fs::write( target, atlas_content ).expect( "" );
            fs::remove_file( &file ).expect( "" );
            continue;
        }
        fs::rename( &file, &target ).expect( "" );
    }
}

fn fix_cursed_celia() {
    let source = "assets\\spine\\character\\Celia\\THe_Curse\\";
    let spine_files: Vec<PathBuf> = fs::read_dir( source ).unwrap().map(|res| res.unwrap().path() ).collect();
    for file in spine_files {
        let file_name = file.file_name().unwrap().to_string_lossy();
        let file_name_vec: Vec< &str > = file_name.split( "." ).collect();

        let target = file.to_str().unwrap().replace( "101601", "060401" );

        if file_name_vec[ 1 ] == "atlas" {
            let mut atlas_content = fs::read_to_string( &file ).expect( "" );
            atlas_content = atlas_content.replace( "101601", "060401" );
            fs::write( target, atlas_content ).expect( "" );
            fs::remove_file( &file ).expect( "" );
            continue;
        }
        fs::rename( &file, &target ).expect( "" );
    }

    fs::rename( "assets\\ui\\costume_face\\illust_inven_char101601.png", "assets\\ui\\costume_face\\illust_inven_char060401.png" ).expect( "" );
    fs::rename( "assets\\ui\\costume_skill_face\\illust_skill_char101601.png", "assets\\ui\\costume_skill_face\\illust_skill_char060401.png" ).expect( "" );
    fs::rename( "assets\\ui\\costume_icon\\icon_costume101601.png", "assets\\ui\\costume_icon\\icon_costume060401.png" ).expect( "" );
}

fn fix_b_rank_idol_helena() {
    let source = "assets\\spine\\skill_cutscene\\Helena\\B-Rank_Idol\\";
    let spine_files: Vec<PathBuf> = fs::read_dir( source ).unwrap().map(|res| res.unwrap().path() ).collect();
    for file in spine_files {
        let file_name = file.file_name().unwrap().to_string_lossy();
        let file_name_vec: Vec< &str > = file_name.split( "." ).collect();

        let target = file.to_str().unwrap().replace( "Char", "char" );

        if file_name_vec[ 1 ] == "atlas" {
            let mut atlas_content = fs::read_to_string( &file ).expect( "" );
            atlas_content = atlas_content.replace( "Char" , "char" );
            fs::remove_file( &file ).expect( "" );
            fs::write( target, atlas_content ).expect( "" );
            continue;
        }
        fs::rename( &file, &target ).expect( "" );
    }
}

fn fix_rising_star_helena() {
    let source = "assets\\spine\\skill_cutscene\\Helena\\Rising_Star\\";
    let spine_files: Vec<PathBuf> = fs::read_dir( source ).unwrap().map(|res| res.unwrap().path() ).collect();
    for file in spine_files {
        let file_name = file.file_name().unwrap().to_string_lossy();
        let file_name_vec: Vec< &str > = file_name.split( "." ).collect();

        let target = file.to_str().unwrap().replace( "_A", "" );

        if file_name_vec[ 1 ] == "atlas" {
            let mut atlas_content = fs::read_to_string( &file ).expect( "" );
            atlas_content = atlas_content.replace( "char061092_A" , "char061092" );
            fs::remove_file( &file ).expect( "" );
            fs::write( target, atlas_content ).expect( "" );
            continue;
        }
        fs::rename( &file, &target ).expect( "" );
    }
}

fn fix_costume_icon_ids() {
    let ids = vec![ 101, 201, 202, 204, 301, 401, 501, 601 ];
    let base_path = "assets\\ui\\costume_icon\\icon_costume";
    for id in ids {
        let new_id = format!( "{:06}", id );
        let from = format!( "{}{}.png", base_path, id );
        let to = format!( "{}{}.png", base_path, new_id );
        fs::rename( from, to ).expect( "Costume Icon Id Fix Error" );
    }
}

fn fix_eff_wallpapers() {
    let remove_files = vec![
        "bg_idcard_bg_eff_1.png",
        "bg_idcard_bg_eff_2.png",
        "bg_idcard_bg_eff_3.png"
    ];
    for file in remove_files {
        let path = format!( "{}{}", "assets\\ui\\wallpapers\\", file );
        let _ = fs::remove_file( path );
    }
}

use skill_icon_extractor::add;
fn extract_skill_icons() {
    println!( "{}", add(2, 2) )
}


