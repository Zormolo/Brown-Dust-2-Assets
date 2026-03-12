# Brown-Dust-2-Assets

<img src="readmeImages/title.png" style="display: block; margin-left: auto; margin-right: auto"> 

<br>

This Repo archives the Assets from the Game: Brown Dust 2. More focused on the Spine Models, Illustrations and Icons.

## Content

- [Spine Models](#spine-models)
  - [Character Models](#character-models)
  - [Skill Cutscene Models](#skill-cutscenes)
  - [Interaction Models](#interaction-models)
  - [NPC Models](#npc-models)
  - [Light Novel Talk Models](#light-novel-talk-models)
- [UI](#ui)
  - [Costume Faces](#costume-faces)
  - [Costume Icons](#costume-icons)
  - [Costume Skill Faces](#costume-skill-faces)
  - [Costume Skill Backgrounds](#costume-skill-backgrounds)
  - [Wallpapers](#wallpapers)
- [Known Issues](#known-stuff-i-need-to-fix)
- [Report Issues](#report-issues)
- [3rd Party](#3rd-party-dependencies)
  - [Unity Assest Extractor](#assets-extractor)

## Spine Models
### Character Models

<img src="readmeImages/spine_character_example.png" style="display: block; margin-left: auto; margin-right: auto">  

<br>

The character models are sorted by Character Name and Costume. If a Character only has one costume all the needed files are directly in the Character folder.  
Here are only the character models included that are from playable / upcoming costumes of the current patch.  
Prestige Skin Character Models are included in the Character Folder.  

[Go to Character Models](assets/spine/character/)

### Skill Cutscenes

<img src="readmeImages/spine_skill_cutscene_example_a.png" style="display: block; margin-left: auto; margin-right: auto">  
<img src="readmeImages/spine_skill_cutscene_example_b.png" style="display: block; margin-left: auto; margin-right: auto">  

<br>

The skill cutscene models are sorted by Charater name and Costume. If a Character only has one costume all the needed files are directly in the Character folder.  
Not every Character has a cutscene therefore some characters are not in there.  
The models dont have the background backed in. Those will be added later in a separate folder unsorted.  
Prestige Skin Skill Cutscene Models are included in the Character Folder.  

[Go to Skill Cutscene Models](assets/spine/skill_cutscene/)

### Interaction Models

<img src="readmeImages/spine_interaction_example.png" style="display: block; margin-left: auto; margin-right: auto">  

<br>

The interaction models are soreted by Character and Costume.  
Those models have the background backed in. So there will be no extra folder for the backgrounds.  
Prestige Skin Interactions are included in the Character folder.

[Go to Interaction Models](assets/spine/interaction/)

### NPC Models

<img src="readmeImages/spine_npc_example.png" style="display: block; margin-left: auto; margin-right: auto">  

<br>

The NPC's from all shops, modes or events that have a spine model are sorted by character name.  

[Go to NPC Models](assets/spine/npc/)

### Light Novel Talk Models

<img src="readmeImages/spine_light_novel_talk_example_a.png" style="display: block; margin-left: auto; margin-right: auto">  
<br>
<img src="readmeImages/spine_light_novel_talk_example_b.png" style="display: block; margin-left: auto; margin-right: auto">  

<br>

The Light Novel Talk Models are mostly used in the Story Pack 1 to 10 after that its rare that characters get such a model.  
In the example with Eclipse the pose of those models are less dynamic and most of them get a back view.  

[Go to Light Novel Talk Models](assets/spine/light_novel_talk/)

## UI
### Costume Faces

<img src="readmeImages/ui_costume_faces_example.png" style="display: block; margin-left: auto; margin-right: auto">  

<br>

All faces of the costumes used in the Companion Menu.  
Those faces are only from playable costumes, skins, temporary companions or summons.

[Go to Costume Faces](assets/ui/costume_face/)

### Costume Skill Faces

<img src="readmeImages/ui_costume_skill_faces_example.png" style="display: block; margin-left: auto; margin-right: auto">  

<br>

All faces of the costumes used in the battle after a skill has been activated.  
Those faces are only from playable costumes, skins, temporary companions or summons.

[Go to Costume Skill Faces](assets/ui/costume_skill_face/)


### Costume Skill Backgrounds

<img src="readmeImages/ui_skill_cutscene_background_example.png" style="display: block; margin-left: auto; margin-right: auto">  

<br>

All backgrounds of the costumes used in the spine animation of the skill cutscenes.  
Those background images are only from playable units with skill cutscenes.  
For now they are just dumped into on folder unsorted.  

[Go to Costume Skill Faces](assets/ui/skill_cutscene_background//)

### Costume Icons

<img src="readmeImages/ui_costume_icons_example.png" style="display: block; margin-left: auto; margin-right: auto">  

<br>

All icons of the costumes used in the Bag Menu under Costume or in the Collection under Costumes.  
Those icons are only from playable costumes, skins or temporary companions.

[Go to Costume Icons](assets/ui/costume_icon/)

### Wallpapers

<img src="readmeImages/ui_wallpaper_example.png" style="display: block; margin-left: auto; margin-right: auto">  

<br>

All the Wallpapers from Story Packs, Character Packs, Content Packs, Event Packs, Skill Cutscenes and Special Cutscenes from Story or Events.

[Go to Wallpapers](//assets/ui/wallpapers/)

## Known Issues

Sword Queen Sylvia & Kind Liberator Samay are cursed if you open them now -> Reason extracted PNG has diff Size then in the atlas file expected, if you want to open then just resize them to the size writen in the atlas file  
I will do a check later that fixed those sizing issues

## Report Issues

If you find any other weird file naming or files that doesnt match the default pattern of that section pls open an issue  
The same goes for stuff you like to have included in the repo  

Make a Issue [here](https://github.com/Zormolo/Brown-Dust-2-Assets/issues)

I will ofc add more stuff  
but what is there atm is the most important for most ppl  
I will also add the audio files when i have a good CLI client to extract them from the bank-files

## 3rd Party Dependencies
### Assets Extractor

The extractor is made by Aelurum. Version 1.2.3 of the CLI is used.  
Link to Repo --> https://github.com/aelurum/AssetStudio