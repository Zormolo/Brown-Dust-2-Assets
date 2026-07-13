from pathlib import Path
import json

OUTPUT_FILE_PATH = Path( '.', 'asset_extractor', 'json', 'bundles.json' )


#######################################################################################################################

def filterBundleTypes( bundles ):
  bundleTypeFilters = (
    # 'spritetexture',
    # 'animation',
    # 'animator',
    # 'prefab',
    # 'cinematimeline',
    # 'scenes',
    # 'effect',
    # 'mesh',
    # 'material',
    # 'meterial',
    # 'sound',
    # 'voice',
    # 'timeline',
    # 'shader',
    # 'map.bundle',
    # 'char.bundle',
    # 'char_bossmonster',
    # 'projectile'
  )
  filteredBundles = []

  for bundle in bundles:
    if any( bundleTypeFilter in bundle for bundleTypeFilter in bundleTypeFilters ):
      continue
    filteredBundles.append( bundle.replace( '\n', '' ) )

  return filteredBundles


#######################################################################################################################

def updateJson( jsonData, bundleSet, bundles ):
  newFoldersFound = 0

  for bundle in bundles:
    folder = bundle[ -32: ]
    name = bundle[ 0: -33 ]

    if folder in bundleSet:
      continue

    assetType = 'texture'
    if 'sound' in name or 'voice' in name:
      assetType = 'audio'

    jsonData[ folder ] = {
      'content': name,
      'type': assetType,
      'lastExtracted': ''
    }
    newFoldersFound += 1

  folderString = 'folder'
  if newFoldersFound > 1:
    folderString = 'folders'
  print( f'{ newFoldersFound } new { folderString } found!' )

  return  {
    'data': jsonData,
    'updates': newFoldersFound
  }


#######################################################################################################################

def makeBundleJson( decodedBundleStrings: list[ str ], updateFile: bool ):
  with open( OUTPUT_FILE_PATH, 'r' ) as file:
    fileContentString = file.read()
  jsonData = json.loads( fileContentString )
  extracedBundlesSet = set( jsonData.keys() )

  filteredBundles = filterBundleTypes( decodedBundleStrings )
  updateResult = updateJson( jsonData, extracedBundlesSet, filteredBundles )

  if updateFile and updateResult[ 'updates' ] > 0:
    with open( OUTPUT_FILE_PATH, 'w' ) as file:
      file.write( json.dumps( updateResult[ 'data' ], indent=2 ) )
    print( 'JSON was updated!' )
  else:
    print( 'Bundle-Json was not updated!' )
  
  return updateResult[ 'data' ]