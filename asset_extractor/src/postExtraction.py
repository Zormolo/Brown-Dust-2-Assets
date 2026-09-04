import os
from pathlib import Path
import re
import shutil
import json
from PIL import Image

extractedFilePaths = []

assetFolderStructure = {
  "spine": {
    "character": {
      'regex': r'^char[0-6][\d_c]*\.(?:png|atlas|skel)',
      'files': []
    },
    "interaction": {
      'regex': r'^illust_dating[\d_]*\.(?:png|atlas|skel)',
      'files': []
    },
    "light_novel_talk": {
      'regex': r'^illust_talk[_\d]*\.(?:png|atlas|skel)',
      'files': []
    },
    "npc": {
      'regex': r'^npc[_ellin|\d]*\.(?:png|atlas|skel)',
      'files': []
    },
    "skill_cutscene": {
      'regex': r'^cutscene_char[\d_a]*\.(?:png|atlas|skel)',
      'files': []
    },
    "special_animation": {
      'regex': r'^(specialillust)?(illust_special)?(illust_pack)?(story_pack)?[\d_]*\.(?:png|atlas|skel)',
      'files': []
    },
    "miscellaneous": {
      'regex': r'^(avatarbodyaccessory)?(colosseumskip)?(pvpskip)?(event_bt)?(Interaction)?(RhythmHitAnim)?[\d_]*\.(?:png|atlas|skel)',
      'files': []
    }
  },
  "ui": {
    "costume_face": {
      'regex': r'^illust_inven_char[\d_c]*\.png',
      'files': []
    },
    "costume_icon": {
      'regex': r'^icon_costume[\d_]*\.png',
      'files': []
    },
    "costume_skill_face": {
      'regex': r'^illust_skill_char[\d_]*\.png',
      'files': []
    },
    "skill_icons": {
      'regex': r'^skillicon[\d_]*\.png',
      'files': []
    },
    "speech_bubble_faces": {
      'regex': r'^illust_(npc)?face[\d_]*\.png',
      'files': []
    },
    "wallpapers": {
      'regex': r'^(bg_idcard_bg|bg_home_wallpaper|bg_guild|bg_goldencolosseum|bg_homedefault)[\d_a-z]*\.png',
      'files': []
    }
  },
  "chibis": {
    'regex': r'^Char[01][\d]{5}_(GetItem|Idle|Move|Rest|Sit|Talent|Victory)?_[\S]*\.png',
    'files': []
  }
}

compiledRegex = []


#######################################################################################################################

def precompailRegex():
  for category in assetFolderStructure.keys():
    if category == 'chibis':
      folderData = assetFolderStructure[ 'chibis' ]
      compiledRegex.append( ( re.compile( folderData["regex"], re.IGNORECASE ), folderData["files"] ) )
    else:
      compiledRegex.extend(
        ( re.compile( entry["regex"], re.IGNORECASE ), entry["files"] )
        for entry in assetFolderStructure[ category ].values()
      )


#######################################################################################################################

def findExtractedFilePaths( currentPath: Path ):
  if os.path.isfile( currentPath ):
    extractedFilePaths.append( currentPath )
    return
  for listEntry in os.listdir( currentPath ):
    newPath = Path( currentPath, listEntry )
    findExtractedFilePaths( newPath )


#######################################################################################################################

def addArchiveFiles():
  archiveFolderPath = Path( 'archive' )
  for archiveFile in os.listdir( archiveFolderPath ):
    archiveFilePath = Path( archiveFolderPath, archiveFile )
    extractedFilePaths.append( archiveFilePath )


#######################################################################################################################

def addSkillIconFiles():
  skillIconFolderPath = Path( 'skillIcons' )
  for skillIconFile in os.listdir( skillIconFolderPath ):
    skillIconFilePath = Path( skillIconFolderPath, skillIconFile )
    extractedFilePaths.append( skillIconFilePath )


#######################################################################################################################

def filterPaths( writeDebugFile: bool ):
    assetFolderStructure[ 'ui' ][ 'skill_cutscene_background' ] = []
    for filePath in extractedFilePaths:
      filePathString = filePath.__str__()
      if "#" in filePathString or not filePathString or "dummy" in filePathString:
        continue

      fileName = filePath.parts[ -1 ]
      if 'Skillbackground_1' in filePath.parts:
        assetFolderStructure[ 'ui' ][ 'skill_cutscene_background' ].append( filePath )
        continue

      for regex, fileList in compiledRegex:
        if regex.search( fileName ):
          fileList.append( filePath )
          break
      # if ".wav" in filePathString:
      #   wavFiles.append( filePath )
      #   continue
      # if ".bytes" in filePathString:
      #   bankFiles.append( filePath )
      #   continue


#######################################################################################################################

def loadMapping():
  mappingFilePath = Path( 'asset_extractor', 'json', 'mapping.json' )
  with open( mappingFilePath, 'r' ) as file:
    fileContentString = file.read()
  return json.loads( fileContentString )


#######################################################################################################################

def createAssetFolder( assetFolderPath: Path ):
  if os.path.exists( assetFolderPath ):
    shutil.rmtree( assetFolderPath )
  os.mkdir( assetFolderPath )

  for subFolder in assetFolderStructure.keys():
    subFolderPath = Path( assetFolderPath, subFolder )
    os.mkdir( subFolderPath )


#######################################################################################################################

def getMappingId( fileName: str, separator: str, maxCount: int ):
  separatorCount = fileName.count( separator )

  if separatorCount <= maxCount:
    return fileName

  mappingId = ''
  for part in fileName.split( separator )[ 0 : -1 ]:
    if not mappingId:
      mappingId = part
      continue
    mappingId += f'{ separator }{ part }'
  return mappingId


#######################################################################################################################

def mapCharacterSpines( assetFolderPath: Path, map: dict ):
  folderPath = Path( assetFolderPath, 'spine', 'character' )
  os.mkdir( folderPath )
  for filePath in assetFolderStructure[ 'spine' ][ 'character' ][ 'files' ]:
    fileName = filePath.stem

    if filePath.parts[ -1 ] == 'char000201_1.png':
      continue

    if 'char101601' in fileName:
      fileName = fileName.replace( 'char101601', 'char060401' )

    if '.skel' in fileName:
      fileName = fileName.replace( '.skel', '' )
    mappingValue = map.get( getMappingId( fileName, '_', 0 ) )

    if mappingValue == None:
      shutil.copy( filePath, folderPath )
      continue

    mappingFolderPath = Path( folderPath, mappingValue )
    if not os.path.exists( mappingFolderPath ):
      os.makedirs( mappingFolderPath )
    newPath = Path( mappingFolderPath, f'{ fileName }{ filePath.suffix }' )

    if os.path.exists( newPath ):
      continue
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapInteractionSpines( assetFolderPath: Path, map: dict ):
  folderPath = Path( assetFolderPath, 'spine', 'interaction' )
  os.mkdir( folderPath )
  for filePath in assetFolderStructure[ 'spine' ][ 'interaction' ][ 'files' ]:
    fileName = filePath.stem

    mappingValue = map.get( getMappingId( fileName, '_', 1 ) )

    if mappingValue == None:
      shutil.copy( filePath, folderPath )
      continue

    mappingFolderPath = Path( folderPath, mappingValue )
    if not os.path.exists( mappingFolderPath ):
      os.makedirs( mappingFolderPath )
    newPath = Path( mappingFolderPath, f'{ fileName }{ filePath.suffix }' )

    if os.path.exists( newPath ):
      continue
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapLightNovelTalkSpines( assetFolderPath: Path, map: dict ):
  folderPath = Path( assetFolderPath, 'spine', 'light_novel_talk' )
  os.mkdir( folderPath )
  for filePath in assetFolderStructure[ 'spine' ][ 'light_novel_talk' ][ 'files' ]:
    fileName = filePath.stem
    mappingValue = map.get( getMappingId( fileName, '_', 1 ) )

    if mappingValue == None:
      shutil.copy( filePath, folderPath )
      continue

    mappingFolderPath = Path( folderPath, mappingValue )
    if not os.path.exists( mappingFolderPath ):
      os.makedirs( mappingFolderPath )
    newPath = Path( mappingFolderPath, f'{ fileName }{ filePath.suffix }' )

    if os.path.exists( newPath ):
      continue
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapNpcSpines( assetFolderPath: Path, map: dict ):
  folderPath = Path( assetFolderPath, 'spine', 'npc' )
  os.mkdir( folderPath )
  for filePath in assetFolderStructure[ 'spine' ][ 'npc' ][ 'files' ]:
    fileName = filePath.stem
    mappingValue = map.get( getMappingId( fileName, '_', 1 ) )
    if mappingValue == None:
      shutil.copy( filePath, folderPath )
      continue

    mappingFolderPath = Path( folderPath, mappingValue )
    if not os.path.exists( mappingFolderPath ):
      os.makedirs( mappingFolderPath )

    newPath = Path( mappingFolderPath, f'{ fileName }{ filePath.suffix }' )
    if os.path.exists( newPath ):
      continue
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapSkillCutsceneSpines( assetFolderPath: Path, map: dict ):
  folderPath = Path( assetFolderPath, 'spine', 'skill_cutscene' )
  os.mkdir( folderPath )
  for filePath in assetFolderStructure[ 'spine' ][ 'skill_cutscene' ][ 'files' ]:
    fileName = filePath.stem

    # Weird Summer Nebris PNG Files Skipping
    if 'char061303' in fileName:
      continue

    if 'char061092_A' in fileName:
      fileName = fileName.replace( '_A', '' )

    mappingValue = map.get( getMappingId( fileName, '_', 1 ) )

    if mappingValue == None:
      shutil.copy( filePath, folderPath )
      continue

    skill_cutscene_layer = filePath.parts[ -2 ]
    if len( skill_cutscene_layer ) > 20:
      mappingValue += f'\\{ skill_cutscene_layer }'

    mappingFolderPath = Path( folderPath, mappingValue )
    if not os.path.exists( mappingFolderPath ):
      os.makedirs( mappingFolderPath )
    newPath = Path( mappingFolderPath, f'{ fileName.lower() }{ filePath.suffix }' )

    if os.path.exists( newPath ):
      continue
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapSpecialAnimatioSpines( assetFolderPath: Path ):
  folderPath = Path( assetFolderPath, 'spine', 'special_animation' )
  os.mkdir( folderPath )
  for filePath in assetFolderStructure[ 'spine' ][ 'special_animation' ][ 'files' ]:
    fileName = filePath.stem

    newPath = Path( folderPath, f'{ fileName.lower() }{ filePath.suffix }' )

    if os.path.exists( newPath ):
      continue
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapMiscellaneousSpines( assetFolderPath: Path ):
  folderPath = Path( assetFolderPath, 'spine', 'miscellaneous' )
  os.mkdir( folderPath )
  allowedAvatarBodyAccessory = set( [ 'avatarbodyaccessory_1006', 'avatarbodyaccessory_1007' ] )
  for filePath in assetFolderStructure[ 'spine' ][ 'miscellaneous' ][ 'files' ]:
    fileName = filePath.stem

    if 'avatarbodyaccessory' in fileName and not fileName in allowedAvatarBodyAccessory:
      continue

    newPath = Path( folderPath, f'{ fileName }{ filePath.suffix }' )
    if os.path.exists( newPath ):
      continue
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapCostumeFace( assetFolderPath: Path ):
  folderPath = Path( assetFolderPath, 'ui', 'costume_face' )
  os.mkdir( folderPath )
  for filePath in assetFolderStructure[ 'ui' ][ 'costume_face' ][ 'files' ]:
    fileName = filePath.stem[ 0 : 23 ]

    if fileName == 'illust_inven_char101601':
      fileName = 'illust_inven_char060401'
    if 'Censorship' in filePath.parts:
      fileName = f'{ fileName }_c'
    fileExtension = filePath.suffix
    newPath = Path( folderPath, f"{ fileName }{ fileExtension }" )
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapCostumeIcon( assetFolderPath: Path ):
  folderPath = Path( assetFolderPath, 'ui', 'costume_icon' )
  os.mkdir( folderPath )

  toShortIds = {
    'icon_costume101_',
    'icon_costume201_',
    'icon_costume202_',
    'icon_costume204_',
    'icon_costume301_',
    'icon_costume401_',
    'icon_costume501_',
    'icon_costume601_'
  }

  for filePath in assetFolderStructure[ 'ui' ][ 'costume_icon' ][ 'files' ]:
    im = Image.open( filePath )
    width, height = im.size

    if width < 200 or height < 200:
      continue

    fileName = filePath.stem
    if 'icon_costume001103_' in fileName:
      continue

    if any( shortId in fileName for shortId in toShortIds ):
      fileName = fileName.replace( 'costume', 'costume000' )

    if 'icon_costume101601' in fileName:
      fileName = fileName.replace( 'icon_costume101601', 'icon_costume060401' )

    fileExtension = filePath.suffix
    newPath = Path( folderPath, f"{ fileName[ 0 : 18 ] }{ fileExtension }" )
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapCostumeSkillFace( assetFolderPath: Path ):
  folderPath = Path( assetFolderPath, 'ui', 'costume_skill_face' )
  os.mkdir( folderPath )
  for filePath in assetFolderStructure[ 'ui' ][ 'costume_skill_face' ][ 'files' ]:
    fileName = filePath.stem
    if fileName == 'illust_skill_char020101_126' or 'Censorship' in filePath.parts:
      continue

    fileName = fileName[ 0 : 23 ]
    if fileName == 'illust_skill_char101601':
      fileName = 'illust_skill_char060401'

    fileExtension = filePath.suffix
    newPath = Path( folderPath, f"{ fileName }{ fileExtension }" )
    shutil.copy( filePath, newPath )

#######################################################################################################################

def mapSkillCutsceneBackgrounds( assetFolderPath: Path ):
  folderPath = Path( assetFolderPath, 'ui', 'skill_cutscene_background' )
  os.mkdir( folderPath )
  for filePath in assetFolderStructure[ 'ui' ][ 'skill_cutscene_background' ]:
    fileName = filePath.stem

    fileExtension = filePath.suffix
    newPath = Path( folderPath, f"{ fileName }{ fileExtension }" )
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapSkillIcons( assetFolderPath: Path ):
  folderPath = Path( assetFolderPath, 'ui', 'skill_icons' )
  os.mkdir( folderPath )
  for filePath in assetFolderStructure[ 'ui' ][ 'skill_icons' ][ 'files' ]:
    fileName = filePath.stem

    fileExtension = filePath.suffix
    newPath = Path( folderPath, f"{ fileName }{ fileExtension }" )
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapSpeechBubbleFaces( assetFolderPath: Path ):
  folderPath = Path( assetFolderPath, 'ui', 'speech_bubble_faces' )
  os.mkdir( folderPath )
  notAllowed = set( [
    'illust_face800001_32',
    'illust_face00440101_1260',
    'illust_face00440106_1261',
    'illust_face00440107_1262',
    'illust_npcface00040172_963',
    'illust_npcface0000070172_850',
    'illust_npcface0000080172_851',
    'illust_npcface00080172_1752',
    'illust_npcface0000130101_906',
    'illust_npcface0000140101_907',
    'illust_npcface0000170101_918',
    'illust_npcface0016090101_1126',
    'illust_npcface0016090101_1126_1',
    'illust_npcface81460101_1658',
    'illust_npcface4000260172_1106',
    'illust_npcface4000270172_1107',
    'illust_npcface4000290101_1109',
    'illust_npcface4000300101_1110',
    'illust_npcface4000310101_1111',
    'illust_npcface4000350101_1112',
    'illust_npcface4000360101_1113',
    'illust_npcface4000370101_1114',
    'illust_npcface4000510101_1127',
    'illust_npcface4000710172_1246',
    'illust_npcface4000720172_1247',
    'illust_npcface4000720172_1248',
    'illust_npcface4000720172_1249',
    'illust_npcface4000720172_1250',
    'illust_npcface4000730172_1248',
    'illust_npcface4000740172_1249',
    'illust_npcface4000750172_1250',
    'illust_npcface4000920172_1366',
    'illust_npcface4001060172_1482',
    'illust_npcface4001070172_1483',
    'illust_npcface4001080172_1484',
    'illust_npcface4001240172_1655',
    'illust_npcface4001250172_1656',
    'illust_npcface4001260172_1657',
    'illust_npcface4001380172_1681',
    'illust_npcface4001390172_1682',
    'illust_npcface4001400172_1683',
    'illust_npcface4001440172_1680',
    'illust_npcface40004101177_1125'
  ] )
  for filePath in assetFolderStructure[ 'ui' ][ 'speech_bubble_faces' ][ 'files' ]:
    fileName = filePath.stem

    if fileName in notAllowed:
      continue

    fileExtension = filePath.suffix
    newPath = Path( folderPath, f"{ fileName }{ fileExtension }" )
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapWallpapers( assetFolderPath: Path ):
  folderPath = Path( assetFolderPath, 'ui', 'wallpapers' )
  os.mkdir( folderPath )
  for filePath in assetFolderStructure[ 'ui' ][ 'wallpapers' ][ 'files' ]:
    fileName = filePath.stem

    if '_eff_' in fileName:
      continue

    fileExtension = filePath.suffix
    newPath = Path( folderPath, f"{ fileName }{ fileExtension }" )
    shutil.copy( filePath, newPath )


#######################################################################################################################

def mapChibis( assetFolderPath: Path, map: dict ):
  folderPath = Path( assetFolderPath, 'chibis' )
  notAllowed = set( [
    'char000302',
    'char000404',
    'char001103',
    'char002901',
    'char050201'
  ] )
  for filePath in assetFolderStructure[ 'chibis' ][ 'files' ]:
    fileName = filePath.stem

    mappingId = fileName.split( '_' )[ 0 ].lower()
    if mappingId in notAllowed:
      continue
    mappingValue = map.get( mappingId )

    if mappingValue == None:
      shutil.copy( filePath, folderPath )
      continue

    mappingFolderPath = Path( folderPath, mappingValue )
    if not os.path.exists( mappingFolderPath ):
      os.makedirs( mappingFolderPath )
    newPath = Path( mappingFolderPath, f'{ fileName.lower() }{ filePath.suffix }' )
    shutil.copy( filePath, newPath )


#######################################################################################################################

def fixLastHopeLoenCharacter( assetFolderPath: Path ):
  fromPath = Path( assetFolderPath, 'spine', 'npc', 'Loen' )
  toPath = Path( assetFolderPath, 'spine', 'character', 'Loen', 'Last_Hope' )
  os.makedirs( toPath )
  for fileName in os.listdir( fromPath ):
    fromFilePath = Path( fromPath, fileName )
    toFileName = fromFilePath.parts[ -1 ].replace( 'npc300501', 'char003201' )
    toFilePath =  Path( toPath, toFileName )
    shutil.copy( fromFilePath, toFilePath )


#######################################################################################################################

def fixAtlasFiles( assetFolderPath: Path ):
  folderPath = Path( assetFolderPath, 'spine' )
  atlasFiles = [
    {
      'path': Path( folderPath, 'character', 'Celia', 'The_Curse', 'char060401.atlas' ),
      'from': 'char101601',
      'to': 'char060401'
    },
    {
      'path': Path( folderPath, 'character', 'Gray', 'B-Rank_Manager', 'char000402.atlas' ),
      'from': '.skel.png',
      'to': '.png'
    },
    {
      'path': Path( folderPath, 'character', 'Loen', 'Last_Hope', 'char003201.atlas' ),
      'from': 'npc300501',
      'to': 'char003201'
    },
    {
      'path': Path( folderPath, 'skill_cutscene', 'Helena', 'B-Rank_Idol', 'cutscene_char061002.atlas' ),
      'from': 'Char061002',
      'to': 'char061002'
    },
    {
      'path': Path( folderPath, 'skill_cutscene', 'Helena', 'Rising_Star', 'cutscene_char061092.atlas' ),
      'from': 'char061092_A',
      'to': 'char061092'
    }
  ]
  for atlasFile in atlasFiles:
    with open( atlasFile[ 'path' ], 'r' ) as file:
      fileContent = file.read()

    fileContent = fileContent.replace( atlasFile[ 'from' ], atlasFile[ 'to' ] )

    with open( atlasFile[ 'path' ], 'w' ) as file:
      file.write( fileContent )


#######################################################################################################################

def mapFilesToAssetFolder( extractionFolderPath: Path, assetFolderPath: Path, writeDebugFile: bool ):
  precompailRegex()
  findExtractedFilePaths( extractionFolderPath )
  addArchiveFiles()
  addSkillIconFiles()
  filterPaths( writeDebugFile )
  mappingJson = loadMapping()
  createAssetFolder( assetFolderPath )

  mapCharacterSpines( assetFolderPath, mappingJson[ 'character' ] )
  mapInteractionSpines( assetFolderPath, mappingJson[ 'interaction' ] )
  mapLightNovelTalkSpines( assetFolderPath, mappingJson[ 'light_novel_talk' ] )
  mapNpcSpines( assetFolderPath, mappingJson[ 'npc' ] )
  mapSkillCutsceneSpines( assetFolderPath, mappingJson[ 'skill_cutscene' ] )
  mapSpecialAnimatioSpines( assetFolderPath )
  mapMiscellaneousSpines( assetFolderPath )

  mapCostumeFace( assetFolderPath )
  mapCostumeIcon( assetFolderPath )
  mapCostumeSkillFace( assetFolderPath )
  mapSkillCutsceneBackgrounds( assetFolderPath )
  mapSkillIcons( assetFolderPath )
  mapSpeechBubbleFaces( assetFolderPath )
  mapWallpapers( assetFolderPath )

  mapChibis( assetFolderPath, mappingJson[ 'character' ] )

  fixLastHopeLoenCharacter( assetFolderPath )
  fixAtlasFiles( assetFolderPath )