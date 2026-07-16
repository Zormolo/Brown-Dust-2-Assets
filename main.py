from pathlib import Path

from asset_extractor.third_party.Myssal_Catalog_Decoder.decoder import decode_catalog_key_data_strings

from asset_extractor.src.makeBundleJson import makeBundleJson
from asset_extractor.src.extractFiles import extracedAssets
from asset_extractor.src.postExtraction import mapFilesToAssetFolder

CATALOG_FILE_PATH = r'E:\Neowiz\Browndust2\BrownDust2_10000002\BrownDust II_Data\StreamingAssets\aa\catalog.json'
EXTRACTION_FOLDER_PATH = Path( '.', 'output' )


#######################################################################################################################

if __name__ == '__main__':
  decodedBundleStrings = decode_catalog_key_data_strings( CATALOG_FILE_PATH )
  bundleJson = makeBundleJson( decodedBundleStrings, True )
  extracedAssets( bundleJson, EXTRACTION_FOLDER_PATH )
  mapFilesToAssetFolder( EXTRACTION_FOLDER_PATH, Path( "assets" ), False )