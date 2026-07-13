from pathlib import Path
import os
import subprocess
import shutil
from concurrent.futures import ThreadPoolExecutor, as_completed

BUNDLE_AGES = {
  1: 'all',
  2: 'new',
  3: 'old'
}

BUNDLE_TYPES = {
  1: 'all',
  2: 'texture',
  3: 'audio'
}

MAX_THREADS = 20

#######################################################################################################################

def getMode():
  print( '' )

  print( 'Which Bundles do you want to extracted?' )
  print( '1 - All' )
  print( '2 - Only New Bundles' )
  print( '3 - Only Old Bundles' )
  bundleAge = int( input( 'Input: ' ).strip() or '3' )

  print( '' )

  print( 'Which Bundle-Type do you want to extraced?' )
  print( '1 - All' )
  print( '2 - Only Textures' )
  print( '3 - Only Audios' )
  bundleType = int( input( 'Input: ' ).strip() or '1' )

  print( '' )

  return { 
    'age': BUNDLE_AGES.get( bundleAge ),
    'type': BUNDLE_TYPES.get( bundleType )
  }


#######################################################################################################################

def filterBundle( bundleData, filterOptions ):
  keys = bundleData.keys()
  age = filterOptions[ 'age' ]
  type = filterOptions[ 'type' ]

  if age == 'all' and type == 'all':
    return keys

  filteredBundles = []

  for key in keys:
    bundleInfo = bundleData[ key ]
    isRightAge = False
    isRightType = False

    if age == 'all':
      isRightAge = True
    elif age == 'new' and not bundleInfo[ 'lastExtracted' ]:
      isRightAge = True
    elif age == 'old' and bundleInfo[ 'lastExtracted' ]:
      isRightAge = True

    if type == 'all':
      isRightType = True
    elif type == bundleInfo[ 'type' ]:
      isRightType = True
    elif type == bundleInfo[ 'type' ]:
      isRightType = True

    if isRightAge and isRightType:
      filteredBundles.append( key )

  return filteredBundles


#######################################################################################################################

def getBundlePath( path, bundleSet ):
  bundlePaths = []

  for folder in os.listdir( path ):
    bundlePath = Path( path, folder )

    if os.path.isdir( bundlePath ):
      if folder in bundleSet:
        bundlePaths.append( bundlePath )
      else:
        result = getBundlePath( bundlePath, bundleSet )
        bundlePaths.extend( result )

  return bundlePaths


#######################################################################################################################

def searchBundlePaths( bundleData, filterOptions ):
  basePath = r'E:\\Gamfs_BrownDust II'
  filteredBundles = filterBundle( bundleData, filterOptions )
  bundleSet = set( filteredBundles )
  return getBundlePath( basePath, bundleSet )


#######################################################################################################################

def deleteExtractionDir( folderPath: Path ):
  if os.path.exists( folderPath ):
    print( 'Delete existing Extraction-Folder...' )
    shutil.rmtree( folderPath )
    print( 'Deletion done...' )


#######################################################################################################################

def runExtraction( cmd ):
  result = subprocess.run(
    cmd,
    shell=True,
    capture_output=True,
    text=True
  )
  return {
    'command': cmd,
    'returnCode': result.returncode,
    'stderr': result.stderr
  }


#######################################################################################################################

def extract( bundlePaths, extractionFolderPath: Path ):
  assetExtractorPath = Path( '.', 'asset_extractor', 'third_party', 'ArknightsStudioCLI', 'ArknightsStudioCLI.exe' )
  extractionCommands = []

  for path in bundlePaths:

    folderName = path.parts[ -1 ]
    outputPath = Path( extractionFolderPath, folderName )
    extractionCommands.append( [
      assetExtractorPath,
      path.__str__(),
      '-o',
      outputPath.__str__(),
      '--asset-type',
      'tex2d,textAsset,audio',
      '--unity-version',
        '2022.3.22f1'
    ] )

  finishedCommands = 0
  maxCommand = len( extractionCommands )

  with ThreadPoolExecutor( max_workers=MAX_THREADS ) as executor:
    threads = [ executor.submit( runExtraction, cmd ) for cmd in extractionCommands ]

    for thread in as_completed( threads ):

      finishedCommands += 1
      print( f'\r{ finishedCommands } / { maxCommand } Folders extracted!', end='', flush=True )
      result = thread.result()

      if result[ 'returnCode' ] != 0:
        print( f'Command: { result[ 'command' ] }' )
        print( f'Exit Code: { result[ 'returnCode' ] }' )
        print( f'Error: { result[ 'stderr' ] }' )


#######################################################################################################################

def extracedAssets( bundleData, extractionFolderPath: Path ):
  inputResult = getMode()
  bundlePaths = searchBundlePaths( bundleData, inputResult )
  print( len( bundlePaths ) )
  deleteExtractionDir( extractionFolderPath )
  extract( bundlePaths, extractionFolderPath )
  print( '\nExtraction complete...' )