from pathlib import Path
import os

from asset_extractor.third_party.Myssal_Catalog_Decoder.decoder import decode_catalog_key_data_strings

from asset_extractor.src.makeBundleJson import makeBundleJson
from asset_extractor.src.extractFiles import extractAssets
from asset_extractor.src.extractSkillIcons import filterSkillIconBundles, extractSkillIcons
from asset_extractor.src.postExtraction import mapFilesToAssetFolder

CATALOG_FILE_PATH = r'E:\Neowiz\Browndust2\BrownDust2_10000002\BrownDust II_Data\StreamingAssets\aa\catalog.json'
EXTRACTION_FOLDER_PATH = Path( '.', 'output' )

assetFolderPaths = []
skillIconFolderPaths = []

#######################################################################################################################

def searchBundlePaths( path, targetFolderNames, deeps = 0 ):
  for entry in os.listdir( path ):
    currentPath = Path( path, entry )
    if os.path.isdir( currentPath ):
      if entry in targetFolderNames:
        skillIconFolderPaths.append( currentPath )
        continue
      if deeps == 0:
        assetFolderPaths.append( currentPath )
      searchBundlePaths( currentPath, targetFolderNames, deeps + 1 )

#######################################################################################################################

if __name__ == '__main__':
  decodedBundleStrings = decode_catalog_key_data_strings( CATALOG_FILE_PATH )
  # makeBundleJson( decodedBundleStrings, True )
  skillIconFolderNames = filterSkillIconBundles( decodedBundleStrings )
  searchBundlePaths( r'E:\\Gamfs_BrownDust II', skillIconFolderNames )
  # extractAssets( EXTRACTION_FOLDER_PATH )
  # extractSkillIcons( assetFolderPaths, decodedBundleStrings )
  # mapFilesToAssetFolder( EXTRACTION_FOLDER_PATH, Path( "assets" ), False )